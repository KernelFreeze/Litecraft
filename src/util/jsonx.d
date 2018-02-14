/*
 * Copyright 2011-2016 Gian Merlino
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

module util.jsonx;

import std.algorithm : find;
import std.ascii : isControl, isUpper, isDigit, isHexDigit, isWhite;
import std.conv;
import std.range;
import std.traits;
import std.exception : enforceEx;
import std.variant;
import std.stdio;

public:

/* JsonValue is currently implemented as a Variant */
alias Variant JsonValue;

struct JsonNull { /* empty type... */ }

/* Encode to a string in memory */
R jsonEncode(R = string, T)(T obj) if (isSomeString!R) {
    auto app = appender!R;
    jsonEncode_impl(obj, app);
    return app.data;
}

/* Encode to any output range */
R jsonEncode(R, T)(T obj, R range) if (isOutputRange!(R, dchar)) {
    jsonEncode_impl(obj, range);
    return range;
}

T jsonDecode(T = JsonValue, R)(R input) if (isInputCharRange!R) {
    auto val = jsonDecode_impl!T(input);
    if (!input.empty)
        throw new JsonException("Garbage at end of stream.");
    return val;
}

private:

template aaKeyType(A) if (isAssociativeArray!A) {
    static if (is(typeof({ A a; return a.keys.front; }()) K)) {
        alias K aaKeyType;
    }
    else
        static assert(0);
}

unittest {
    static assert(is(aaKeyType!(int[string]) == string));
    static assert(is(aaKeyType!(int[dstring]) == dstring));
    static assert(is(aaKeyType!(int[float]) == float));
}

template isInputCharRange(R) {
    enum isInputCharRange = isInputRange!R && isSomeChar!(ElementType!R);
}

auto nextChar(R)(ref R input) pure {
    if (input.empty)
        throw new JsonException("Premature end of input");

    static if (isSomeString!R) {
        /* Don't bother decoding UTF */
        return input[0];
    }
    else {
        return input.front;
    }
}

void skipChar(R)(ref R input) {
    static if (isSomeString!R) {
        /* Don't bother decoding UTF */
        input = input[1 .. $];
    }
    else {
        input.popFront;
    }
}

void enforceChar(R)(ref R input, char c, bool sw) if (isInputCharRange!R) {
    auto nextChar = nextChar(input);
    if (nextChar != c)
        throw new JsonException("Expected " ~ to!string(c) ~ ", saw " ~ to!string(nextChar));

    skipChar(input);
    if (sw)
        skipWhite(input);
}

void skipWhite(R)(ref R input) if (isInputCharRange!R) {
    static if (isSomeString!R) {
        /* Don't bother decoding UTF */
        while (!input.empty && isWhite(input[0])) {
            input = input[1 .. $];
        }
    }
    else {
        while (!input.empty && isWhite(input.front)) {
            input.popFront;
        }
    }
}

/* Encode JsonValue. Not able to encode all variants, but should be able to round-trip
 * variants created from jsonDecode. */
void jsonEncode_impl(T : JsonValue, A)(T v, ref A app) {
    if (v.type() == typeid(string)) {
        jsonEncode_impl(v.get!string, app);
    }
    else if (v.type() == typeid(JsonValue[])) {
        jsonEncode_impl(v.get!(JsonValue[]), app);
    }
    else if (v.type() == typeid(JsonValue[string])) {
        jsonEncode_impl(v.get!(JsonValue[string]), app);
    }
    else if (v.type() == typeid(real)) {
        jsonEncode_impl(v.get!real, app);
    }
    else if (v.type() == typeid(bool)) {
        jsonEncode_impl(v.get!bool, app);
    }
    else if (v.type() == typeid(JsonNull)) {
        jsonEncode_impl(v.get!JsonNull, app);
    }
    else {
        throw new JsonException("Can't encode Variant with type " ~ to!string(v.type()));
    }
}

/* Encode string */
void jsonEncode_impl(T, A)(T str, ref A app) if (isSomeString!T) {
    app.put('"');

    /* Iterate dchars so we get unicode points as units */
    foreach (dchar c; str) {
        if (c == '\b') {
            app.put(`\b`);
        }
        else if (c == '\f') {
            app.put(`\f`);
        }
        else if (c == '\n') {
            app.put(`\n`);
        }
        else if (c == '\r') {
            app.put(`\r`);
        }
        else if (c == '\t') {
            app.put(`\t`);
        }
        else if (c == '"' || c == '\\' || c == '/') {
            app.put('\\');
            app.put(c);
        }
        else if (isControl(c)) {
            /* Do unicode escape */
            app.put(`\u`);
            foreach (i; retro(iota(4))) {
                /* Nybble at position i */
                auto n = (c >> (i * 4)) & 0x0F;
                auto hex = n < 10 ? '0' + n : 'A' + n - 10;
                app.put(cast(char) hex);
            }
        }
        else {
            app.put(c);
        }
    }

    app.put('"');
}

/* Encode character */
void jsonEncode_impl(T, A)(T val, ref A app) if (!is(T == enum) && isSomeChar!T) {
    jsonEncode_impl(to!string(val), app);
}

/* Encode number, bool */
void jsonEncode_impl(T, A)(T val, ref A app)
        if (!is(T == enum) && (isNumeric!T || is(T == bool))) {
    app.put(to!string(val));
}

/* Encode enum */
void jsonEncode_impl(T, A)(T val, ref A app) if (is(T == enum)) {
    jsonEncode_impl(to!string(val), app);
}

/* Encode JsonNull */
void jsonEncode_impl(T, A)(T val, ref A app) if (is(T == JsonNull)) {
    app.put("null");
}

/* Encode struct or class */
void jsonEncode_impl(S, A)(S obj, ref A app)
        if ((is(S == struct) || is(S == class)) && !is(S == JsonNull)) {
    static if (is(S == class)) {
        /* A class could be null */
        if (obj is null) {
            app.put("null");
            return;
        }
    }

    app.put('{');
    bool first = true;

    foreach (i, val; obj.tupleof) {
        if (!first)
            app.put(',');
        first = false;

        /* obj.tupleof[i].stringof is something like "obj.member".
         * We just want "member" */
        auto key = obj.tupleof[i].stringof[4 .. $];

        jsonEncode_impl(key, app);
        app.put(':');
        jsonEncode_impl(val, app);
    }

    app.put('}');
}

/* Encode array */
void jsonEncode_impl(S : T[], T, A)(S arr, ref A app) if (!isSomeString!S) {
    app.put('[');
    bool first = true;

    foreach (item; arr) {
        if (!first)
            app.put(',');
        jsonEncode_impl(item, app);
        first = false;
    }

    app.put(']');
}

/* Encode associative array */
void jsonEncode_impl(S : T[K], T, K, A)(S arr, ref A app) {
    app.put('{');
    bool first = true;

    // XXX provide a way to disable sorting
    foreach (key; arr.keys.sort) {
        if (!first)
            app.put(',');

        static if (isSomeString!K) {
            /* Encoding a string key, we can do it directly */
            jsonEncode_impl(key, app);
        }
        else {
            /* Encoding a non-string key. Since JSON keys must be strings,
             * we must coerce the key to a string before encoding. */
            jsonEncode_impl(to!string(key), app);
        }

        app.put(':');
        jsonEncode_impl(arr[key], app);
        first = false;
    }

    app.put('}');
}

/* Decode anything -> JsonValue */
Variant jsonDecode_impl(T : JsonValue, R)(ref R input) if (isInputCharRange!R) {
    JsonValue v;

    if (input.empty)
        throw new JsonException("Premature end of input.");

    dchar c = input.front;
    if (c == '"') {
        v = jsonDecode_impl!string(input);
    }
    else if (c == '[') {
        v = jsonDecode_impl!(JsonValue[])(input);
    }
    else if (c == '{') {
        v = jsonDecode_impl!(JsonValue[string])(input);
    }
    else if (c == '-' || (c >= '0' && c <= '9')) {
        v = jsonDecode_impl!real(input);
    }
    else if (c == 't' || c == 'f') {
        v = jsonDecode_impl!bool(input);
    }
    else if (c == 'n') {
        v = jsonDecode_impl!JsonNull(input);
    }
    else {
        throw new JsonException("Can't decode into JsonValue");
    }

    return v;
}

/* Decode JSON object -> D associative array, class, or struct */
T jsonDecode_impl(T, R)(ref R input)
        if (isInputCharRange!R && (is(T == struct) || is(T == class)
            || (isAssociativeArray!T)) && !is(T : JsonNull)) {
    auto first = true;

    static if (is(T == class)) {
        auto obj = new T;

        /* Classes can be null */
        if (!input.empty && input.front == 'n') {
            jsonDecode_impl!JsonNull(input);
            return null;
        }
    }
    else static if (is(T == struct) || isAssociativeArray!T) {
        T obj;
    }
    else
        static assert(0);

    /* First character should be '{' */
    enforceChar(input, '{', true);

    while (!input.empty) {
        if (input.front == '}') {
            /* } is the last character */
            input.popFront;
            return obj;
        }

        if (!first) {
            /* All key/value pairs after the first should be preceded by commas */
            enforceChar(input, ',', true);
        }

        /* Read key */
        static if (isAssociativeArray!T) {
            /* Decode into the correct key type for T */
            auto key = jsonDecode_impl!(aaKeyType!T)(input);
        }
        else {
            auto key = jsonDecode_impl!string(input);
        }

        skipWhite(input);

        /* Read colon */
        enforceChar(input, ':', true);

        /* Determine type of value */
        static if (isAssociativeArray!T) {
            /* Arrays are composed of only one type */
            auto ckey = cast(KeyType!T) key;
            obj[ckey] = jsonDecode_impl!(typeof(obj[ckey]))(input);
        }
        else {
            /* Get class and struct members from tupleof */
            bool didRead = false;

            foreach (i, oval; obj.tupleof) {
                /* obj.tupleof[i].stringof is something like "obj.member".
                 * We just want "member" */
                if (key == obj.tupleof[i].stringof[4 .. $]) {
                    /* Assigning to oval doesn't seem to work, but obj.tupleof[i] does */
                    obj.tupleof[i] = jsonDecode_impl!(typeof(obj.tupleof[i]))(input);
                    didRead = true;
                    break;
                }
            }

            if (!didRead) {
                /* eek. Read the value and toss it */
                jsonDecode_impl!JsonValue(input);
            }
        }

        skipWhite(input);
        first = false;
    }

    /* Premature end of input */
    throw new JsonException("premature end of input");
    assert(0);
}

/* Decode JSON array -> D array */
T[] jsonDecode_impl(A : T[], T, R)(ref R input)
        if (isInputCharRange!R && !isSomeString!A) {
    auto first = true;
    auto app = appender!(T[]);

    /* First character should be '[' */
    enforceChar(input, '[', true);

    while (!input.empty) {
        if (input.front == ']') {
            /* ] is the last character */
            input.popFront;
            return app.data;
        }

        if (!app.data.empty) {
            /* All values after the first should be preceded by commas */
            enforceChar(input, ',', true);
        }

        /* Read value */
        app.put(jsonDecode_impl!T(input));
        skipWhite(input);
    }

    /* Premature end of input */
    throw new JsonException("premature end of input");
    assert(0);
}

/* Decode JSON number -> D number */
T jsonDecode_impl(T, R)(ref R input)
        if (isInputCharRange!R && isNumeric!T && !is(T == enum)) {
    /* Attempt decoding of JSON strings into D numbers
     * by ignoring surrounding quote marks if present */
    auto first = nextChar(input);
    if (first == '"')
        skipChar(input);

    try {
        auto number = parse!T(input);

        /* If we started with a quote mark, we need to end with one */
        if (first == '"')
            enforceChar(input, '"', false);

        return number;
    }
    catch (ConvException e) {
        /* Convert ConvException into JsonException */
        throw new JsonException("ConvException: " ~ e.msg);
    }
}

/* Decode JSON string -> D string */
T jsonDecode_impl(T, R)(ref R input) if (isInputCharRange!R && isSomeString!T) {
    auto app = Appender!T();

    /* For strings we can attempt to scan without copying or decoding UTF */
    enum canReuseInput = is(T == R);
    static if (canReuseInput) {
        /* If inputSave is set, it means we don't yet need to copy */
        auto inputSave = input.save;
    }

    /* First character should be '"' */
    enforceChar(input, '"', false);

    while (!input.empty) {
        static if (canReuseInput) {
            Unqual!(typeof(input[0])) c = input[0];
        }
        else {
            dchar c = input.front;
        }

        if (c == '"') {
            /* End of string */
            input.popFront;

            static if (canReuseInput) {
                if (inputSave)
                    return inputSave[1 .. inputSave.length - input.length - 1];
            }

            return app.data;
        }
        else if (c == '\\') {
            /* Escape sequence */

            static if (canReuseInput) {
                /* We need to use the appender */
                if (inputSave) {
                    app = Appender!T(inputSave[1 .. inputSave.length - input.length]);
                    inputSave = null;
                }
            }

            /* Advance to escaped character */
            input.popFront;
            if (input.empty)
                throw new JsonException("Premature end of input.");
            static if (canReuseInput) {
                c = input[0];
            }
            else {
                c = input.front;
            }

            switch (c) {
            case '"':
            case '\\':
            case '/':
                app.put(c);
                input.popFront;
                break;
            case 'b':
                app.put('\b');
                input.popFront;
                break;
            case 'f':
                app.put('\f');
                input.popFront;
                break;
            case 'n':
                app.put('\n');
                input.popFront;
                break;
            case 'r':
                app.put('\r');
                input.popFront;
                break;
            case 't':
                app.put('\t');
                input.popFront;
                break;
            case 'u':
                /* Unicode escape coming up */
                input.popFront;

                /* Function to read the next 4 hex digits from "input" into a wchar */
                wchar nextUnit() {
                    wchar unit = 0;

                    foreach (i; retro(iota(4))) {
                        if (input.empty)
                            throw new JsonException("Encountered EOF inside unicode escape.");

                        /* Read hex digit */
                        dchar hex = input.front;
                        if (!hex.isHexDigit)
                            throw new JsonException(
                                    "Encountered non-hex digit inside unicode escape.");

                        /* Convert to number */
                        auto val = isDigit(hex) ? hex - '0' : isUpper(hex)
                            ? hex - 'A' + 10 : hex - 'a' + 10;

                        /* Fill in the nybble */
                        unit |= (val << (i * 4));

                        /* Advance stream */
                        input.popFront;
                    }

                    return unit;
                }

                /* Unicode escape state */
                wchar[2] units;

                /* Read first unit */
                units[0] = nextUnit;
                if (units[0] < 0xD800 || units[0] > 0xD8FF) {
                    /* Only one utf16 code unit needed */
                    app.put(units[0]);
                }
                else {
                    /* units[0] is the first half of a two-unit utf16 code */
                    /* Expect another \u */
                    enforceChar(input, '\\', false);
                    enforceChar(input, 'u', false);

                    /* Read next unit */
                    units[1] = nextUnit;

                    /* units.front will return a dchar merging both units */
                    app.put(units.front);
                }

                break;

            default:
                throw new JsonException("encountered bogus escape sequence");
            }
        }
        else if (isControl(c)) {
            /* Error - JSON strings cannot include raw control characters */
            throw new JsonException("encountered raw control character");
        }
        else {
            /* Regular character */
            static if (canReuseInput) {
                if (!inputSave)
                    app.put(c);
                input = input[1 .. $];
            }
            else {
                app.put(c);
                input.popFront;
            }
        }
    }

    /* Premature end of input */
    throw new JsonException("Premature end of input.");
    assert(0);
}

/* Decode JSON string -> char, enum */
T jsonDecode_impl(T, R)(ref R input)
        if (isInputCharRange!R && (isSomeChar!T || is(T == enum))) {
    return to!T(jsonDecode_impl!string(input));
}

/* Decode JSON bool -> D bool */
bool jsonDecode_impl(T, R)(ref R input) if (isInputCharRange!R && is(T == bool)) {
    if (input.empty)
        throw new JsonException("Premature end of input.");

    dchar c = input.front;
    if (c == 't') {
        input.popFront;
        enforceChar(input, 'r', false);
        enforceChar(input, 'u', false);
        enforceChar(input, 'e', false);
        return true;
    }
    else if (c == 'f') {
        input.popFront;
        enforceChar(input, 'a', false);
        enforceChar(input, 'l', false);
        enforceChar(input, 's', false);
        enforceChar(input, 'e', false);
        return false;
    }

    assert(0);
}

/* Decode JSON null -> D null */
JsonNull jsonDecode_impl(T, R)(ref R input)
        if (isInputCharRange!R && is(T == JsonNull)) {
    if (input.empty)
        throw new JsonException("Premature end of input.");
    enforceChar(input, 'n', false);
    enforceChar(input, 'u', false);
    enforceChar(input, 'l', false);
    enforceChar(input, 'l', false);
    return JsonNull();
}

class JsonException : Exception {
    this(string s) @safe pure nothrow {
        super(s);
    }
}

unittest {
    /* String decodes */
    assert(jsonDecode(`""`) == "");
    assert(jsonDecode(`"\u0391 \u0392\u0393\t\u03B3\u03b4"`) == "\u0391 \u0392\u0393\t\u03B3\u03B4");
    assert(jsonDecode(`"\uD834\uDD1E"`) == "\U0001D11E");
    assert(jsonDecode("\"\U0001D11E and \u0392\"") == "\U0001D11E and \u0392");
}

unittest {
    /* String encodes */
    assert(jsonEncode("he\u03B3l\"lo") == "\"he\u03B3l\\\"lo\"");
    assert(jsonEncode("\U0001D11E and \u0392") == "\"\U0001D11E and \u0392\"");
}

unittest {
    /* Mix string/dstring encode and decode */
    string narrowStr = "\"\\uD834\\uDD1E \U0001D11E\"";
    dstring wideLoad = "\"\\uD834\\uDD1E \U0001D11E\"";
    assert(jsonDecode!string(wideLoad) == "\U0001D11E \U0001D11E");
    assert(jsonDecode!dstring(wideLoad) == "\U0001D11E \U0001D11E");
    assert(jsonDecode!string(narrowStr) == "\U0001D11E \U0001D11E");
    assert(jsonDecode!dstring(narrowStr) == "\U0001D11E \U0001D11E");
    assert(jsonEncode!string(jsonDecode!string(wideLoad)) == "\"\U0001D11E \U0001D11E\"");
    assert(jsonEncode!dstring(jsonDecode!string(wideLoad)) == "\"\U0001D11E \U0001D11E\"");
    assert(jsonEncode!string(jsonDecode!dstring(wideLoad)) == "\"\U0001D11E \U0001D11E\"");
    assert(jsonEncode!dstring(jsonDecode!dstring(wideLoad)) == "\"\U0001D11E \U0001D11E\"");
    /* Decode associative array indexed by dstring */
    narrowStr = "{" ~ narrowStr ~ ": 3}";
    wideLoad = "{" ~ wideLoad ~ ": 3}";
    auto dstringAA1 = jsonDecode!(int[dstring])(narrowStr);
    auto dstringAA2 = jsonDecode!(int[dstring])(wideLoad);
    assert(dstringAA1["\U0001D11E \U0001D11E"] == 3);
    assert(dstringAA2["\U0001D11E \U0001D11E"] == 3);
}

unittest {
    /* Decode JSON strings into D numbers */
    assert(jsonDecode!int(`"34"`) == 34);
}

unittest {
    /* Deep associative array encode/decode */
    int[string][uint][string] daa;
    daa["foo"][2]["baz"] = 4;
    auto daaStr = jsonEncode(daa);
    assert(daaStr == `{"foo":{"2":{"baz":4}}}`);
    assert(jsonDecode!(int[string][uint][string])(daaStr)["foo"][2]["baz"] == 4);
}

version (unittest) {
    private static struct MyConfig {
        string encoding;
        string[] plugins;
        int indent = 2;
        bool indentSpaces;
    }

    private static class X {
        enum foos {
            Bar,
            Baz
        };
        real[] reals;
        int[string] ints;
        MyConfig conf;
        foos foo;
        void qux() {
        }
    }

    private auto xjson = `{
        "foo" : "Baz",
        "reals" : [ 3.4, 7.2e+4, 5, 0, -33 ],
        "ints" : { "one": 1, "two": 2 },
        "bogus" : "ignore me",
        "conf" : {
            "encoding" : "UTF-8",
            "indent" : 4,
            "plugins" : [ "perl", "d" ],
            "indentSpaces" : true
        }
    }`;
}
unittest {
    /* Structured decode into user-defined type */
    auto x = jsonDecode!X(`null`);
    assert(x is null);
}

unittest {
    auto x = jsonDecode!X(`{}`);
    assert(x !is null);
    assert(x.conf.indent == 2);
    assert(x.foo == X.foos.Bar);
}

unittest {
    auto x = jsonDecode!X(xjson);
    assert(x !is null);
    assert(x.foo == X.foos.Baz);
    assert(x.reals == [3.4L, 72000, 5, 0, -33]);
    assert(x.ints["one"] == 1);
    assert(x.ints["two"] == 2);
    assert(x.conf.encoding == "UTF-8");
    assert(x.conf.plugins == ["perl", "d"]);
    assert(x.conf.indent == 4);
    assert(x.conf.indentSpaces == true);
    /* Structured encode */
    assert(jsonEncode(x) == `{"reals":[3.4,72000,5,0,-33],"ints":{"one":1,"two":2},"conf":{"encoding":"UTF-8","plugins":["perl","d"],"indent":4,"indentSpaces":true},"foo":"Baz"}`);
}

unittest {
    /* Structured decode into JsonValue */
    auto xv = jsonDecode(`null`);
    assert(xv.type() == typeid(JsonNull));
}

unittest {
    auto xv = jsonDecode(xjson);
    assert(xv["bogus"] == "ignore me");
    assert(xv["foo"] == "Baz");
    assert(xv["reals"][0] == 3.4L);
    assert(xv["reals"][1] == 72000L);
    assert(xv["reals"][2] == 5L);
    assert(xv["reals"][3] == 0L);
    assert(xv["reals"][4] == -33L);
    assert(xv["ints"]["two"] == 2);
    assert(xv["ints"]["two"] == 2);
    assert(xv["conf"]["encoding"] == "UTF-8");
    assert(xv["conf"]["plugins"][0] == "perl");
    assert(xv["conf"]["plugins"][1] == "d");
    assert(xv["conf"]["indent"] == 4);
    assert(xv["conf"]["indentSpaces"] == true);
    /* Encode JsonValue back to JSON */
    assert(jsonEncode(xv) == `{"bogus":"ignore me","conf":{"encoding":"UTF-8","indent":4,"indentSpaces":true,"plugins":["perl","d"]},"foo":"Baz","ints":{"one":1,"two":2},"reals":[3.4,72000,5,0,-33]}`);
}

unittest {
    /* All truncated streams should be errors */
    foreach (i; iota(xjson.length)) {
        bool caught;

        if (i < xjson.length) {
            caught = false;
            try {
                jsonDecode(xjson[0 .. i]);
            }
            catch (JsonException) {
                caught = true;
            }
            assert(caught);

            caught = false;
            try {
                jsonDecode!X(xjson[0 .. i]);
            }
            catch (JsonException) {
                caught = true;
            }
            assert(caught);
        }

        if (i > 0) {
            caught = false;
            try {
                jsonDecode(xjson[i .. $]);
            }
            catch (JsonException) {
                caught = true;
            }
            assert(caught);

            caught = false;
            try {
                jsonDecode!X(xjson[i .. $]);
            }
            catch (JsonException) {
                caught = true;
            }
            assert(caught);
        }
    }
}

unittest {
    /* Tests from std.json */
    auto jsons = [
        `null`, `true`, `false`, `0`, `123`, `-4321`, `0.23`, `-0.23`, `""`, `1.223e+24`, `"hello\nworld"`,
        `"\"\\\/\b\f\n\r\t"`, `[]`, `[12,"foo",true,false]`, `{}`, `{"a":1,"b":null}`, `{"goodbye":[true,"or",false,["test",42,{"nested":{"a":23.54,"b":0.0012}}]],"hello":{"array":[12,null,{}],"json":"is great"}}`
    ];
    foreach (json; jsons) {
        auto v = jsonDecode(json);
        auto rt = jsonEncode(v);
        assert(rt == json, "roundtrip -> " ~ json);
    }
}

unittest {
    /* More tests from std.json */
    auto v = jsonDecode(`"\u003C\u003E"`);
    assert(jsonEncode(v) == "\"\&lt;\&gt;\"");
    v = jsonDecode(`"\u0391\u0392\u0393"`);
    assert(jsonEncode(v) == "\"\&Alpha;\&Beta;\&Gamma;\"");
    v = jsonDecode(`"\u2660\u2666"`);
    assert(jsonEncode(v) == "\"\&spades;\&diams;\"");
}

unittest {
    // Encode/decode enum values
    enum TestEnum {
        A,
        B,
        C
    }

    enum IntEnum : int {
        A = 1,
        B = 2,
        C = 3
    }

    enum CharEnum : char {
        A = 'a',
        B = 'b',
        C = 'c'
    }

    struct EnumStruct {
        TestEnum a;
        IntEnum b;
        CharEnum c;
    }

    auto enumstruct = EnumStruct(TestEnum.A, IntEnum.B, CharEnum.C);
    auto json = enumstruct.jsonEncode();
    assert(json.jsonDecode!EnumStruct == enumstruct);
}

unittest {
    // Character keys in AA
    auto aa = jsonDecode!(int[char])(`{"a": 1, "b": 2, "c": 3}`);
    assert('a' in aa);
    assert('x' !in aa);
    assert(aa['a'] == 1);
    assert(aa['b'] == 2);
}
