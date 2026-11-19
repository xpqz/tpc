{ The Crt unit: a text screen of 80 columns by 25 rows with a cursor, a window, colours and
  the keyboard, as in Turbo Pascal's Crt.

  The state lives here, in Pascal: the window (WindMin and WindMax, as in TP: the 0-based
  column in the low byte and the row in the high byte), the cursor and TextAttr. The host
  only stores and draws cells (docs/abi.md section 8, import `screen`):
    screen(0, column, row, char + attr * 256)               puts a character
    screen(1 or 2, left + top * 256, right + bottom * 256,
           lines + attr * 256)                              scrolls a rectangle up or down
    screen(3, column, row, 0)                               places the cursor
  Every operation takes 0-based screen coordinates; GotoXY and WhereX count from 1 inside the
  window.

  Why the state is kept here and not in the host (decision 39): WhereX, WhereY, WindMin and
  TextAttr are plain variables a program reads without a host call, and the host stays a store
  of cells that any front end (browser canvas, test dump) can show.

  The initialisation hooks Crt into System's console (ConsoleHooked): from then on Write and
  WriteLn draw on the window, and ReadLn reads a line from the keyboard with its own editing:
  Backspace erases the last character, Esc the whole line, extended keys are ignored.

  The path of a Write: the compiler expands it to TextOut calls, System's ConsoleWrite collects
  the bytes until the end of the statement (TextFlush), then ConsoleOut (DrawBytes here) draws
  them one by one (DrawChar) with `screen` calls. The path of a ReadLn: System's
  FillFromConsole calls ConsoleLine (EditLine here), which reads keys with ReadKey, a host
  import that blocks until the page delivers a key. }
unit Crt;

interface

const
  Black = 0;
  Blue = 1;
  Green = 2;
  Cyan = 3;
  Red = 4;
  Magenta = 5;
  Brown = 6;
  LightGray = 7;
  DarkGray = 8;
  LightBlue = 9;
  LightGreen = 10;
  LightCyan = 11;
  LightRed = 12;
  LightMagenta = 13;
  Yellow = 14;
  White = 15;
  Blink = 128;

var
  TextAttr: Byte;
  WindMin, WindMax: Word;
  LastMode: Word;
  CheckBreak, CheckEof, CheckSnow, DirectVideo: Boolean;

procedure ClrScr;
procedure ClrEol;
procedure InsLine;
procedure DelLine;
procedure GotoXY(x, y: Byte);
function WhereX: Byte;
function WhereY: Byte;
procedure Window(x1, y1, x2, y2: Byte);
procedure TextColor(color: Byte);
procedure TextBackground(color: Byte);
procedure HighVideo;
procedure LowVideo;
procedure NormVideo;
function ReadKey: Char;
function KeyPressed: Boolean;
procedure Delay(ms: Word);
procedure Sound(hertz: Word);
procedure NoSound;

implementation

{ Host imports (docs/abi.md section 8). `readkey` blocks until a key is available; at the end
  of scripted keys in the test runner the program stops with exit code 255. `sleep` is Delay. }
procedure ScreenOp(op, a, b, c: LongInt); external 'screen';
function HostReadKey: LongInt; external 'readkey';
function HostKeyPressed: LongInt; external 'keypressed';
procedure HostSleep(ms: LongInt); external 'sleep';

const
  OpPut = 0;
  OpScrollUp = 1;
  OpScrollDown = 2;
  OpCursor = 3;
  Columns = 80;
  Rows = 25;
  StartAttr = $07;
  Bell = 7;
  Backspace = 8;
  LineFeed = 10;
  CarriageReturn = 13;
  Escape = 27;
  Blank = 32;

type
  CharBuffer = array[0..32767] of Char;
  PCharBuffer = ^CharBuffer;

var
  { The cursor, 0-based on the screen. }
  CursorColumn, CursorRow: Integer;

function WindowLeft: Integer;
begin
  WindowLeft := WindMin and $FF
end;

function WindowTop: Integer;
begin
  WindowTop := WindMin shr 8
end;

function WindowRight: Integer;
begin
  WindowRight := WindMax and $FF
end;

function WindowBottom: Integer;
begin
  WindowBottom := WindMax shr 8
end;

procedure ShowCursor;
begin
  ScreenOp(OpCursor, CursorColumn, CursorRow, 0)
end;

procedure PutCell(column, row, ch: Integer);
begin
  ScreenOp(OpPut, column, row, ch + TextAttr * 256)
end;

{ Scrolls the rows top..bottom of the columns left..right by `lines`; the rows left empty are
  blank in the current attribute. Scrolling by the height of the rectangle clears it. }
procedure ScrollRectangle(op, left, top, right, bottom, lines: Integer);
begin
  ScreenOp(op, left + top * 256, right + bottom * 256, lines + TextAttr * 256)
end;

procedure ScrollWindowUp;
begin
  ScrollRectangle(OpScrollUp, WindowLeft, WindowTop, WindowRight, WindowBottom, 1)
end;

{ The cursor moves one row down; below the last row of the window the window scrolls. }
procedure NextRow;
begin
  if CursorRow = WindowBottom then ScrollWindowUp
  else CursorRow := CursorRow + 1
end;

{ One character as the screen shows it: control characters move the cursor, every other byte
  is a glyph. A glyph in the last column goes on in the first column of the next row, so
  writing the bottom right cell of the window scrolls at once. }
procedure DrawChar(ch: Char);
begin
  if Ord(ch) = Bell then Exit;
  if Ord(ch) = Backspace then
  begin
    if CursorColumn > WindowLeft then CursorColumn := CursorColumn - 1
  end
  else if Ord(ch) = LineFeed then NextRow
  else if Ord(ch) = CarriageReturn then CursorColumn := WindowLeft
  else
  begin
    PutCell(CursorColumn, CursorRow, Ord(ch));
    if CursorColumn = WindowRight then
    begin
      CursorColumn := WindowLeft;
      NextRow
    end
    else CursorColumn := CursorColumn + 1
  end
end;

{ System's console writer while Crt is in use. The cursor is placed once per batch of
  characters, not once per character. }
procedure DrawBytes(address, count: LongInt);
var i: LongInt;
begin
  for i := 0 to count - 1 do DrawChar(PCharBuffer(address)^[i]);
  ShowCursor
end;

procedure Echo(ch: Char);
begin
  DrawChar(ch);
  ShowCursor
end;

procedure ClrScr;
begin
  ScrollRectangle(OpScrollUp, WindowLeft, WindowTop, WindowRight, WindowBottom,
    WindowBottom - WindowTop + 1);
  CursorColumn := WindowLeft;
  CursorRow := WindowTop;
  ShowCursor
end;

procedure ClrEol;
var column: Integer;
begin
  for column := CursorColumn to WindowRight do PutCell(column, CursorRow, Blank)
end;

{ A blank row opens at the cursor row; the rows below move down, the last one is lost. }
procedure InsLine;
begin
  ScrollRectangle(OpScrollDown, WindowLeft, CursorRow, WindowRight, WindowBottom, 1)
end;

procedure DelLine;
begin
  ScrollRectangle(OpScrollUp, WindowLeft, CursorRow, WindowRight, WindowBottom, 1)
end;

{ A position outside the window is ignored, as in TP. }
procedure GotoXY(x, y: Byte);
begin
  if (x >= 1) and (y >= 1) and (x <= WindowRight - WindowLeft + 1)
    and (y <= WindowBottom - WindowTop + 1) then
  begin
    CursorColumn := WindowLeft + x - 1;
    CursorRow := WindowTop + y - 1;
    ShowCursor
  end
end;

function WhereX: Byte;
begin
  WhereX := CursorColumn - WindowLeft + 1
end;

function WhereY: Byte;
begin
  WhereY := CursorRow - WindowTop + 1
end;

{ An invalid rectangle is ignored. The cursor goes to the new window's top left corner. }
procedure Window(x1, y1, x2, y2: Byte);
begin
  if (x1 >= 1) and (y1 >= 1) and (x2 <= Columns) and (y2 <= Rows) and (x1 <= x2)
    and (y1 <= y2) then
  begin
    WindMin := (x1 - 1) + (y1 - 1) * 256;
    WindMax := (x2 - 1) + (y2 - 1) * 256;
    CursorColumn := x1 - 1;
    CursorRow := y1 - 1;
    ShowCursor
  end
end;

{ TextAttr: the foreground in bits 0..3, the background in bits 4..6, blink in bit 7. A colour
  above 15 sets blink; TextBackground keeps only three bits. }
procedure TextColor(color: Byte);
begin
  if color > 15 then color := (color and 15) or Blink;
  TextAttr := (TextAttr and $70) or color
end;

procedure TextBackground(color: Byte);
begin
  TextAttr := (TextAttr and $8F) or ((color and 7) shl 4)
end;

procedure HighVideo;
begin
  TextAttr := TextAttr or 8
end;

procedure LowVideo;
begin
  TextAttr := TextAttr and $F7
end;

procedure NormVideo;
begin
  TextAttr := StartAttr
end;

{ The host returns the bytes of ReadKey; an extended key is 0 and then its scan code. }
function ReadKey: Char;
begin
  ReadKey := Chr(HostReadKey)
end;

function KeyPressed: Boolean;
begin
  KeyPressed := HostKeyPressed <> 0
end;

procedure Delay(ms: Word);
begin
  HostSleep(ms)
end;

{ There is no speaker: Sound and NoSound do nothing. }
procedure Sound(hertz: Word);
begin
end;

procedure NoSound;
begin
end;

{ System's console line reader while Crt is in use: reads keys until Enter, echoing them, and
  stores the line at `address`, at most `max` characters. Returns its length. Any other
  control key, and any key beyond `max`, is ignored. }
function EditLine(address, max: LongInt): LongInt;
var key: Char; count: LongInt; entered: Boolean;

  procedure EraseLast;
  begin
    count := count - 1;
    Echo(Chr(Backspace));
    Echo(Chr(Blank));
    Echo(Chr(Backspace))
  end;

begin
  count := 0;
  entered := False;
  repeat
    key := ReadKey;
    if Ord(key) = 0 then key := ReadKey { an extended key: its scan code is dropped }
    else if Ord(key) = CarriageReturn then entered := True
    else if Ord(key) = Backspace then
    begin
      if count > 0 then EraseLast
    end
    else if Ord(key) = Escape then
    begin
      while count > 0 do EraseLast
    end
    else if (Ord(key) >= Blank) and (count < max) then
    begin
      PCharBuffer(address)^[count] := key;
      count := count + 1;
      Echo(key)
    end
  until entered;
  Echo(Chr(CarriageReturn));
  Echo(Chr(LineFeed));
  EditLine := count
end;

begin
  TextAttr := StartAttr;
  WindMin := 0;
  WindMax := (Columns - 1) + (Rows - 1) * 256;
  LastMode := 3;
  CheckBreak := True;
  CheckEof := False;
  CheckSnow := True;
  DirectVideo := True;
  CursorColumn := 0;
  CursorRow := 0;
  { ConsoleHooked goes last: from then on System calls the two hooks. }
  ConsoleOut := DrawBytes;
  ConsoleLine := EditLine;
  ConsoleHooked := True
end.
