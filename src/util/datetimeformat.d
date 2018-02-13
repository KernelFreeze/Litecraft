/** 
*  
*  Contains functions for formatting a SysTime object
*  Original source: https://github.com/cmays90/datetimeformat/blob/master/source/datetimeformat.d
*
**/

module util.datetimeformat;

private import std.datetime;
private import std.ascii : toLower, isDigit, isAlpha;
private import std.utf : validate, toUTF32, toUTF8, toUCSindex, toUTFindex,
    encode;
private import std.string : toStringz, format;
private import std.conv : to;

/// Short (three-letter) Days of the week
immutable string[] SHORT_DAY_NAME = [DayOfWeek.sun : "Sun", "Mon", "Tue",
    "Wed", "Thu", "Fri", "Sat"];

///	Full names of the days of the week.
immutable string[] LONG_DAY_NAME = [
    DayOfWeek.sun : "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
];

///	Short (three-letter) names of the months of the year.
immutable string[] SHORT_MONTH_NAME = [
    Month.jan : "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug",
    "Sep", "Oct", "Nov", "Dec"
];

///	Full names of the months of the year.
immutable string[Month.max + 1] LONG_MONTH_NAME = [
    Month.jan : "January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"
];

/**	Formats dt according to formatString.
 *
 *	Returns:
 *		the formatted date string.
 *	Throws:
 *		SysTimeFormatException  if the formatting fails, e.g. because of an error in the format
 *		                         string.
 *		UtfException             if formatString is not a correctly-formed UTF-8 string.
 */

string format(const SysTime dt, string formatString) {
    validate(formatString);
    return format(dt, dt.dayOfWeek, formatString);
}

string format(const SysTime dt, DayOfWeek dayOfWeek, string formatString) {
    validate(formatString);
    bool nonNull;
    immutable(char)* charPos = toStringz(formatString);
    scope (success)
        assert(*charPos == '\0');

    return format(dt, dayOfWeek, charPos, nonNull, '\0');
}

private {

    // taken from Phobos (where it is private in D2)
    const ubyte[256] UTF8stride = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 0xFF, 0xFF,];

    string format(const SysTime dt, DayOfWeek dayOfWeek,
            ref immutable(char)* charPos, out bool nonNull, char portionEnd) {

        // function uses null-terminated string to make finding the end easier
        long lastNumber = int.min;
        string result;

        while (*charPos != portionEnd) {
            if (beginsElement(*charPos)) {
                bool newNonNull;
                result ~= formatElement(dt, dayOfWeek, charPos, newNonNull, lastNumber);
                if (newNonNull)
                    nonNull = true;
            }
            else if (beginsLiteral(*charPos)) {
                result ~= formatLiteral(charPos);
            }
            else
                switch (*charPos) {
            case '\0': // unclosed portion
                assert(portionEnd == '}');
                throw new SysTimeFormatException(E_UNCLOSED_COLLAPSIBLE);

            case '}':
                throw new SysTimeFormatException(E_UNOPENED_COLLAPSIBLE);

            case ']':
                throw new SysTimeFormatException(E_UNOPENED_FIELD);

            default: // self-literal character
                result ~= *(charPos++);
            }
        }

        return result;
    }

    /*	Processes a single format element.  A format element is any of the following:
	 *	- an alphabetical format specifier
	 *	- a collapsible portion
	 *	- an alignment field
	 *	Literals and alignment field widths are not included.  The purpose is
	 *	to deal with those elements that cannot be part of an alignment field
	 *	padding or width.
	 */
    string formatElement(const SysTime dt, DayOfWeek dayOfWeek,
            ref immutable(char)* charPos, out bool nonNull, ref long lastNumber)
    in {
        assert(beginsElement(*charPos));
    }
    body {
        switch (*charPos) {
        case '[': {
                charPos++;
                string portion = formatField(dt, dayOfWeek, charPos, nonNull);
                charPos++;
                return portion;
            }

        case '{': {
                charPos++;
                string portion = format(dt, dayOfWeek, charPos, nonNull, '}');
                charPos++;
                return nonNull ? portion : null;
            }

        default:
            char letter = cast(char) toLower(*charPos);
            immutable(char)* beginSpec = charPos;

            do {
                ++charPos;
            }
            while (toLower(*charPos) == letter);

            string formatted = formatBySpec(dt, dayOfWeek,
                    beginSpec[0 .. charPos - beginSpec], lastNumber);

            if (formatted.length != 0) {
                nonNull = true;
                return formatted;
            }
            else {
                return null;
            }
        }
    }

    string formatLiteral(ref immutable(char)* charPos)
    in {
        assert(beginsLiteral(*charPos));
    }
    body {
        switch (*charPos) {
        case '`': { // literal character
                if (*++charPos == '\0') {
                    throw new SysTimeFormatException(E_MISSING_LITERAL);
                }
                uint len = UTF8stride[*charPos];
                scope (exit)
                    charPos += len;
                return charPos[0 .. len];
            }

        case '\'': { // literal portion
                immutable(char)* beginLiteral = ++charPos;
                while (*charPos != '\'') {
                    if (*charPos == '\0') {
                        throw new SysTimeFormatException(E_UNCLOSED_LITERAL);
                    }
                    charPos++;
                }
                return beginLiteral[0 .. (charPos++) - beginLiteral];
            }

        default:
            assert(false);
        }
    }

    struct Piece {
        dstring dtext;
        string text;
        ubyte type; // 0 = raw, 1 = literal; 2 = formatted

        @property uint asNumber() {
            if (dtext.length > 9)
                throw new SysTimeFormatException(E_OVERFLOW_WIDTH);
            uint result;
            foreach (c; dtext) {
                assert(c >= '0' && c <= '9');
                result = result * 10 + (c - '0');
            }
            return result;
        }
    }

    string formatField(const SysTime dt, DayOfWeek dayOfWeek,
            ref immutable(char)* charPos, out bool nonNull) {
        Piece[] pieces;

        // first parse the format string within the [...]
        {
            Piece[] tempPieces;
            long lastNumber = int.min;

            while (*charPos != ']') {
                if (beginsElement(*charPos)) {
                    bool newNonNull;
                    tempPieces ~= Piece(null, formatElement(dt, dayOfWeek,
                            charPos, newNonNull, lastNumber), 2);
                    if (newNonNull)
                        nonNull = true;
                }
                else if (beginsLiteral(*charPos)) {
                    tempPieces ~= Piece(null, formatLiteral(charPos), 1);
                }
                else
                    switch (*charPos) {
                case '\0':
                    throw new SysTimeFormatException(E_UNCLOSED_FIELD);

                case '}':
                    throw new SysTimeFormatException(E_UNOPENED_COLLAPSIBLE);

                default: {
                        immutable(char)* begin = charPos;
                        do {
                            charPos++;
                        }
                        while (*charPos != '\0' && *charPos != ']' && *charPos != '}'
                                && !beginsElement(*charPos) && !beginsLiteral(*charPos));

                        tempPieces ~= Piece(null, begin[0 .. charPos - begin], 0);
                    }
                }
            }

            /*	convert tempPieces into a form in which
			 *	- no two consecutive tempPieces have the same type
			 *	- only non-literalised numbers have type 0
			 */
            ubyte lastType = ubyte.max;

            foreach (piece; tempPieces) {
                switch (piece.type) {
                case 0:
                    foreach (dchar c; piece.text) {
                        if (isDigit(c)) {
                            if (lastType == 0) {
                                pieces[$ - 1].dtext ~= c;
                            }
                            else {
                                pieces ~= Piece([c], null, 0);
                                lastType = 0;
                            }
                        }
                        else {
                            if (lastType == 1) {
                                pieces[$ - 1].dtext ~= c;
                            }
                            else {
                                pieces ~= Piece([c], null, 1);
                                lastType = 1;
                            }
                        }
                    }
                    break;

                case 1:
                    if (lastType == 1) {
                        pieces[$ - 1].dtext ~= toUTF32(piece.text);
                    }
                    else {
                        pieces ~= Piece(toUTF32(piece.text), null, 1);
                        lastType = 1;
                    }
                    break;

                case 2:
                    if (lastType == 2) {
                        pieces[$ - 1].text ~= piece.text;
                    }
                    else {
                        pieces ~= piece;
                        lastType = 2;
                    }
                    break;

                default:
                    assert(false);
                }
            }
        }

        if (pieces.length < 2)
            throw new SysTimeFormatException(E_INCOMPLETE_FIELD);

        // detect the field width/padding
        dchar padLeft, padRight;
        size_t fieldWidth = 0;
        bool moreOnRight;

        if (pieces[0].type == 0) {
            // field width on left
            if (pieces[$ - 1].type == 0)
                throw new SysTimeFormatException(E_DOUBLE_WIDTH);

            fieldWidth = pieces[0].asNumber;
            if (fieldWidth == 0)
                throw new SysTimeFormatException(E_ZERO_FIELD);
            if (pieces[1].type != 1)
                throw new SysTimeFormatException(E_INCOMPLETE_FIELD);

            pieces = pieces[1 .. $];
            padLeft = pieces[0].dtext[0];
            pieces[0].dtext = pieces[0].dtext[1 .. $];
            if (pieces[$ - 1].type == 1) {
                padRight = pieces[$ - 1].dtext[$ - 1];
                pieces[$ - 1].dtext.length = pieces[$ - 1].dtext.length - 1;
            }

        }
        else if (pieces[$ - 1].type == 0) {
            // field width on right
            moreOnRight = true;
            fieldWidth = pieces[$ - 1].asNumber;
            if (fieldWidth == 0)
                throw new SysTimeFormatException(E_ZERO_FIELD);
            if (pieces[$ - 2].type != 1)
                throw new SysTimeFormatException(E_INCOMPLETE_FIELD);

            pieces = pieces[0 .. $ - 1];
            padRight = pieces[$ - 1].dtext[$ - 1];
            pieces[$ - 1].dtext.length = pieces[$ - 1].dtext.length - 1;
            if (pieces[0].type == 1) {
                padLeft = pieces[0].dtext[0];
                pieces[0].dtext = pieces[0].dtext[1 .. $];
            }

        }
        else {
            // field width given by number of padding characters
            if (pieces[0].type == 1) {
                padLeft = pieces[0].dtext[0];
                for (fieldWidth = 1; fieldWidth < pieces[0].dtext.length
                        && pieces[0].dtext[fieldWidth] == padLeft; fieldWidth++) {
                }
                pieces[0].dtext = pieces[0].dtext[fieldWidth .. $];
            }
            if (pieces[$ - 1].type == 1) {
                padRight = pieces[$ - 1].dtext[$ - 1];
                ulong pos;
                for (pos = pieces[$ - 1].dtext.length - 1; pos > 0
                        && pieces[$ - 1].dtext[pos - 1] == padRight; pos--) {
                }
                if (pieces[$ - 1].dtext.length - pos > fieldWidth)
                    moreOnRight = true;
                fieldWidth += pieces[$ - 1].dtext.length - pos;
                pieces[$ - 1].dtext.length = pos;
            }
        }

        assert(fieldWidth != 0);

        debug (datetimeformat) {
            writefln("padding chars: %s %s.", padLeft == dchar.init ? "none"
                    : [padLeft], padRight == dchar.init ? "none" : [padRight]);
            writefln("width: %d", fieldWidth);
            writefln("%d pieces", pieces.length);
        }

        // read the field format - now use it
        // but first, concatenate and measure the content
        size_t contentLength;
        string formattedContent;
        foreach (piece; pieces) {
            assert(piece.dtext.length == 0 || piece.text.length == 0);
            if (piece.text.length == 0) {
                formattedContent ~= toUTF8(piece.dtext);
                contentLength += piece.dtext.length;
            }
            else {
                formattedContent ~= piece.text;
                contentLength += toUCSindex(piece.text, piece.text.length);
            }
        }
        debug (datetimeformat)
            writefln("content length %d: %s", contentLength, formattedContent);

        if (contentLength > fieldWidth) {
            throw new SysTimeFormatException(E_FIELD_OVERFLOW);
        }
        if (contentLength >= fieldWidth)
            return formattedContent;
        assert(formattedContent.length == toUTFindex(formattedContent, contentLength));

        // distribute padding
        ulong padWidth = fieldWidth - contentLength, padLeftWidth = 0, padRightWidth = 0;
        if (padLeft == dchar.init) {
            padRightWidth = padWidth;
        }
        else if (padRight == dchar.init) {
            padLeftWidth = padWidth;
        }
        else {
            padLeftWidth = padRightWidth = padWidth / 2;
            if (padWidth % 2 == 1) {
                if (moreOnRight) {
                    padRightWidth++;
                }
                else {
                    padLeftWidth++;
                }
            }
        }
        debug (datetimeformat)
            writefln("Padding distribution: %d %d %d = %d", padLeftWidth,
                    contentLength, padRightWidth, fieldWidth);
        assert(padLeftWidth + contentLength + padRightWidth == fieldWidth);

        // now do it!
        char[] result;

        for (int i = 0; i < padLeftWidth; i++)
            encode(result, padLeft);
        result ~= formattedContent;
        for (int i = 0; i < padRightWidth; i++)
            encode(result, padRight);
        return cast(string) result;
    }

    bool beginsElement(char c) {
        return isAlpha(c) || c == '[' || c == '{';
    }

    bool beginsLiteral(char c) {
        return c == '\'' || c == '`';
    }

    immutable string DIGITS12 = "110123456789";
    /+TEN = DIGITS12[1..3],
					ELEVEN = DIGITS12[0..2],
					TWELVE = DIGITS12[3..5];+/

    /*const E_BAD_UTF
	  = "Error in date/time format string: invalid UTF-8 sequence";*/
    immutable E_MISSING_LITERAL = "Error in date/time format string: missing character after '`'";
    immutable E_UNCLOSED_LITERAL = "Error in date/time format string: unterminated literal portion";
    immutable E_UNCLOSED_FIELD = "Error in date/time format string: '[' without matching ']'";
    immutable E_UNCLOSED_COLLAPSIBLE = "Error in date/time format string: '{' without matching '}'";
    immutable E_UNOPENED_FIELD = "Error in date/time format string: ']' without matching '['";
    immutable E_UNOPENED_COLLAPSIBLE = "Error in date/time format string: '}' without matching '{'";
    immutable E_INCOMPLETE_FIELD = "Error in date/time format string: Incomplete alignment field";
    immutable E_ZERO_FIELD = "Error in date/time format string: Zero-width alignment field";
    immutable E_DOUBLE_WIDTH = "Error in date/time format string: Width of alignment field doubly specified";
    immutable E_OVERFLOW_WIDTH = "Error in date/time format string: Field width too large";
    immutable E_FIELD_OVERFLOW = "Date/time formatting failed: Insufficient field width to hold content";
    immutable E_BC_YY = "Date/time formatting failed: Format 'yy' for BC dates undefined";
    immutable E_INVALID_DATE_TIME = "Date/time formatting failed: Invalid date/time";

    string formatBySpec(const SysTime dt, DayOfWeek dow, string spec, ref long lastNumber) {
        with (dt) switch (spec) {

        case "yy":
            lastNumber = year;
            if (year <= 0) {
                throw new SysTimeFormatException(E_BC_YY);
            }
            return formatTwoDigit(cast(byte)(year % 100));

        case "yyy":
            lastNumber = year;
            return to!string(year > 0 ? year : (lastNumber = 1 - dt.year));

        case "yyyy":
            lastNumber = year;
            return format("%04d", year > 0 ? year : (lastNumber = 1 - dt.year));

        case "YYY":
            lastNumber = year < 0 ? -year : year; // year.min remains the same
            return to!string(year);

        case "b":
            return (year == year.min || year > 0) ? null : "bc";

        case "bb":
            return year > 0 ? "ad" : "bc";

        case "bbb":
            return year > 0 ? "ce" : "bce";

        case "bbbb":
            return (year == year.min || year > 0) ? null : "bce";

        case "B":
            return (year == year.min || year > 0) ? null : "BC";

        case "BB":
            return year > 0 ? "AD" : "BC";

        case "BBB":
            return year > 0 ? "CE" : "BCE";

        case "BBBB":
            return (year == year.min || year > 0) ? null : "BCE";

        case "m":
            lastNumber = month;
            return format12(month);

        case "mm":
            lastNumber = month;
            char[] fmt = new char[2];
            if (month < 10) {
                fmt[0] = '0';
                fmt[1] = cast(char)('0' + month);
            }
            else {
                fmt[0] = '1';
                fmt[1] = cast(char)('0' - 10 + month);
            }
            return cast(string) fmt;

        case "mmm":
            return SHORT_L_MONTH_NAME[month];

        case "Mmm":
            return SHORT_MONTH_NAME[month];

        case "MMM":
            return SHORT_U_MONTH_NAME[month];

        case "mmmm":
            return LONG_L_MONTH_NAME[month];

        case "Mmmm":
            return LONG_MONTH_NAME[month];

        case "MMMM":
            return LONG_U_MONTH_NAME[month];

        case "d":
            lastNumber = day;
            return to!string(day);

        case "dd":
            lastNumber = day;
            return formatTwoDigit(day);

        case "t":
            return ordinalSuffix(lastNumber, false);

        case "T":
            return ordinalSuffix(lastNumber, true);

        case "www":
            return SHORT_L_DAY_NAME[dow];

        case "Www":
            debug (datetimeformat)
                writefln("Day of week: %d", cast(byte) dow);
            return SHORT_DAY_NAME[dow];

        case "WWW":
            return SHORT_U_DAY_NAME[dow];

        case "wwww":
            return LONG_L_DAY_NAME[dow];

        case "Wwww":
            return LONG_DAY_NAME[dow];

        case "WWWW":
            return LONG_U_DAY_NAME[dow];

        case "h":
            lastNumber = hour;
            if (hour == 0) {
                return DIGITS12[3 .. 5];
            }
            else if (hour <= 12) {
                return format12(hour);
            }
            else {
                return format12(hour - 12);
            }

        case "hh":
            lastNumber = hour;
            if (hour == 0) {
                return DIGITS12[3 .. 5];
            }
            else if (hour <= 12) {
                return formatTwoDigit(hour);
            }
            else {
                return formatTwoDigit(hour - 12);
            }

        case "H":
            lastNumber = hour;
            return to!string(hour);

        case "HH":
            lastNumber = hour;
            return formatTwoDigit(hour);

        case "a":
            return hour < 12 ? "a" : "p";

        case "aa":
            return hour < 12 ? "am" : "pm";

        case "A":
            return hour < 12 ? "A" : "P";

        case "AA":
            return hour < 12 ? "AM" : "PM";

        case "i":
            lastNumber = minute;
            return to!string(minute);

        case "ii":
            lastNumber = minute;
            return formatTwoDigit(minute);

        case "s":
            lastNumber = second;
            return to!string(second);

        case "ss":
            lastNumber = second;
            return formatTwoDigit(second);

        case "f":
            lastNumber = fracSecs().total!"msecs" / 100;
            return DIGITS12[lastNumber + 2 .. lastNumber + 3];

        case "ff":
            lastNumber = fracSecs().total!"msecs" / 10;
            return to!string(lastNumber);

        case "FF":
            lastNumber = fracSecs().total!"msecs" / 10;
            return formatTwoDigit(cast(byte) lastNumber);

        case "fff":
            lastNumber = fracSecs().total!"msecs";
            return to!string(fracSecs().total!"msecs");

        case "FFF":
            lastNumber = fracSecs().total!"msecs";
            return format("%03d", fracSecs().total!"msecs");

            /*
			case "zzzz":
				return hour == hour.min ? null :
				  timezone().utcOffsetAt(stdTime) >= 0 ?
				    format("+%02d%02d", timezone().utcOffsetAt(stdTime) / 60,
				      timezone().utcOffsetAt(stdTime) % 60) :
  				  format("-%02d%02d", -timezone().utcOffsetAt(stdTime) / 60,
				      -timezone().utcOffsetAt(stdTime) % 60);
      */

        default:
            throw new SysTimeFormatException(cast(string)(
                    "Error in date/time format string: Undefined format specifier '" ~ spec ~ "'"));
        }
    }

    string formatTwoDigit(int b)
    in {
        assert(b == byte.min || (b >= 0 && b <= 99));
    }
    body {
        if (b == byte.min)
            return null;
        char[] fmt = new char[2];
        fmt[0] = cast(byte)('0' + b / 10);
        fmt[1] = cast(byte)('0' + b % 10);
        return cast(string) fmt;
    }

    string format12(int b)
    in {
        assert(b >= 0);
        assert(b <= 12);
    }
    body {
        switch (b) {
        case 10:
            return DIGITS12[1 .. 3];
        case 11:
            return DIGITS12[0 .. 2];
        case 12:
            return DIGITS12[3 .. 5];
        default:
            return DIGITS12[2 + b .. 3 + b];
        }
    }

    string ordinalSuffix(long lastNumber, bool upperCase) {
        if (lastNumber < 0)
            return null;
        lastNumber %= 100;
        if (lastNumber >= 4 && lastNumber <= 20) {
            return upperCase ? "TH" : "th";
        }
        switch (lastNumber % 10) {
        case 1:
            return upperCase ? "ST" : "st";

        case 2:
            return upperCase ? "ND" : "nd";

        case 3:
            return upperCase ? "RD" : "rd";

        default:
            return upperCase ? "TH" : "th";
        }
    }
}

///	Exception thrown if there was a problem in formatting a date or time.
class SysTimeFormatException : Exception {
    private this(string msg) {
        super(msg);
    }
}

///	Short (three-letter) names of the days of the week.
immutable string[7] SHORT_L_DAY_NAME = [
    DayOfWeek.sun : "sun", "mon", "tue", "wed", "thu", "fri", "sat"
];

///	Short (three-letter) names of the days of the week.
immutable string[7] SHORT_U_DAY_NAME = [
    DayOfWeek.sun : "SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"
];

///	Full names of the days of the week.
immutable string[7] LONG_L_DAY_NAME = [
    DayOfWeek.sun : "sunday", "monday", "tuesday", "wednesday", "thursday", "friday", "saturday"
];

///	Full names of the days of the week.
immutable string[7] LONG_U_DAY_NAME = [
    DayOfWeek.sun : "SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"
];

///	Short (three-letter) names of the months of the year.
immutable string[13] SHORT_L_MONTH_NAME = [
    ['\xFF', '\xFF', '\xFF'
], Month.jan : "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];

///	Short (three-letter) names of the months of the year.
immutable string[13] SHORT_U_MONTH_NAME = [
    ['\xFF', '\xFF', '\xFF'
], Month.jan : "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

///	Full names of the months of the year.
immutable string[13] LONG_L_MONTH_NAME = [
    null, Month.jan : "january", "february", "march", "april", "may", "june",
    "july", "august", "september", "october", "november", "december"
];

///	Full names of the months of the year.
immutable string[13] LONG_U_MONTH_NAME = [
    null, Month.jan : "JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE",
    "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER"
];

unittest {
    import std.stdio;

    writefln("Unittest commenced at %s", Clock.currTime.toString);

    SysTime dt = SysTime(DateTime(2005, 9, 8, 16, 51, 9), dur!"msecs"(427));
    // basic formatting
    assert(dt.format("dd/mm/yy") == "08/09/05");
    assert(dt.format("Www dt Mmm yyyy BB") == "Thu 8th Sep 2005 AD");
    assert(dt.format("h:ii AA") == "4:51 PM");
    assert(dt.format("yyyy-mm-dd HH:ii:ss") == "2005-09-08 16:51:09");
    assert(dt.format("HH:ii:ss.FFF") == "16:51:09.427");
    // alignment fields
    assert(dt.format("[------Wwww.....]") == "--Thursday.");
    assert(dt.format("[11-Wwww.]") == "--Thursday.");
    assert(dt.format("[-----Wwww......]") == "-Thursday..");
    assert(dt.format("[-Wwww.11]") == "-Thursday..");
    assert(dt.format("[9`1Www]") == "111111Thu");
    assert(dt.format("[`1Wwww-10]") == "1Thursday-");
    assert(dt.format("[d/m/yyy           ]HH:ii:ss") == "8/9/2005   16:51:09");

    assert(dt.format("d Mmm yyy{ B}{ HH:ii:ss}") == "8 Sep 2005 16:51:09");
    assert(dt.format("{d }{Mmm }yyy BB") == "8 Sep 2005 AD");
    assert(dt.format("HH:ii{:ss}{.FFF}") == "16:51:09.427");

    assert(dt.format("HH:ii{:ss}{.FFF}") == "16:51:09.427");
    dt.fracSecs(dur!"msecs"(0));
    assert(dt.format("HH:ii{:ss}{.FFF}") == "16:51:09.000");
    dt.second = 0;
    assert(dt.format("HH:ii{:ss}{.FFF}") == "16:51:00.000");
    assert(dt.format("d Mmm yyy{ B}{ HH:ii:ss}") == "8 Sep 2005 16:51:00");
    dt.hour = 0;
    assert(dt.format("d Mmm yyy{ B}{ HH:ii:ss}") == "8 Sep 2005 00:51:00");
    dt.minute = 0;
    assert(dt.format("d Mmm yyy{ B}{ HH:ii:ss}") == "8 Sep 2005 00:00:00");
    assert(dt.format("{d }{Mmm }yyy BB") == "8 Sep 2005 AD");
    dt.month = Month.min;
    assert(dt.format("{d }{Mmm }yyy BB") == "8 Jan 2005 AD");
    dt.day = 1;
    assert(dt.format("{d }{Mmm }yyy BB") == "1 Jan 2005 AD");

    dt.month = Month.sep;
    dt.day = 8;

    // nesting of fields and collapsible portions
    assert(dt.format("[13 Mmmm [d..]]") == " September 8.");
    assert(dt.format("[13 Mmmm{ d}]") == "  September 8");
    dt.day = 1;
    assert(dt.format("[13 Mmmm{ d}]") == "  September 1");
    assert(dt.format("{[13 Mmmm{ d}]}") == "  September 1");
    dt.month = Month.min;
    assert(dt.format("{[13 Mmmm{ d}]}") == "    January 1");
    dt.day = 8;
    assert(dt.format("{[13 Mmmm{ d}]}") == "    January 8");
}
