{ A toy editor standing in for the IDE's modules: it keeps a few lines of text, paints them
  with a menu bar and a status line, and handles the keys the IDE will: typing, Backspace,
  Enter, the cursor keys, ^Y (delete line), the ^K prefix (^K B and ^K K mark a block start
  and end), F9 (compile), Ctrl-F9 (compile and run) and Alt-X (quit). It ends with the
  screen as the last key left it. }
unit Editor;

interface

procedure Edit;

implementation

uses Crt, IdeHost;

const
  MaxLines = 12;
  KeyEnter = $000D;
  KeyBackspace = $0008;
  KeyCtrlK = $000B;
  KeyCtrlY = $0019;
  KeyUp = $4800;
  KeyDown = $5000;
  KeyLeft = $4B00;
  KeyRight = $4D00;
  KeyF9 = $4300;
  KeyF10 = $4400;
  KeyCtrlF9 = $6600;
  KeyAltX = $2D00;

type
  Flat = array[0..1023] of Char;

var
  Lines: array[1..MaxLines] of string[60];
  Count, Row, Col: Integer;
  BlockStart, BlockEnd: Integer;
  Message, Log: string;

{ Sets the status message and keeps every message in the log row above the status line. }
procedure Say(Text: string);
begin
  Message := Text;
  Log := Copy(Log + Text + '; ', 1, 79);
end;

{ A key as one word: the character, or the extended code in the high byte. }
function GetKey: Word;
var
  C: Char;
begin
  C := ReadKey;
  if C = #0 then GetKey := Ord(ReadKey) shl 8 else GetKey := Ord(C);
end;

procedure Paint;
var
  I: Integer;
begin
  TextAttr := $70;
  GotoXY(1, 1);
  Write(' File  Edit  Run  Options', '':54);
  TextAttr := $1E;
  for I := 1 to MaxLines do
  begin
    GotoXY(1, I + 1);
    if I <= Count then Write(Lines[I], '':60 - Length(Lines[I]))
    else Write('':60);
  end;
  TextAttr := $07;
  GotoXY(1, 24);
  Write(Log, '':79 - Length(Log));
  TextAttr := $70;
  GotoXY(1, 25);
  Write(Row:3, ':', Col:2, '  B', BlockStart, ' K', BlockEnd, '  ', Message,
    '':50 - Length(Message));
  GotoXY(Col, Row + 1);
end;

procedure Insert(Ch: Char);
begin
  if Length(Lines[Row]) < 60 then
  begin
    System.Insert(Ch, Lines[Row], Col);
    Inc(Col);
  end;
end;

procedure NewLine;
var
  I: Integer;
  Tail: string;
begin
  if Count = MaxLines then Exit;
  Tail := Copy(Lines[Row], Col, 60);
  Lines[Row][0] := Chr(Col - 1);
  for I := Count downto Row + 1 do Lines[I + 1] := Lines[I];
  Inc(Count);
  Inc(Row);
  Lines[Row] := Tail;
  Col := 1;
end;

procedure Backspace;
begin
  if Col > 1 then
  begin
    Dec(Col);
    System.Delete(Lines[Row], Col, 1);
  end;
end;

procedure DeleteLine;
var
  I: Integer;
begin
  for I := Row to Count - 1 do Lines[I] := Lines[I + 1];
  Lines[Count] := '';
  if Count > 1 then Dec(Count);
  if Row > Count then Row := Count;
  if Col > Length(Lines[Row]) + 1 then Col := Length(Lines[Row]) + 1;
end;

{ The text as the compiler gets it: lines joined by CR LF. }
function FlattenText(var T: Flat): Integer;
var
  I, J, N: Integer;
begin
  N := 0;
  for I := 1 to Count do
  begin
    for J := 1 to Length(Lines[I]) do
    begin
      T[N] := Lines[I][J];
      Inc(N);
    end;
    T[N] := #13;
    T[N + 1] := #10;
    N := N + 2;
  end;
  FlattenText := N;
end;

function Compile: Boolean;
var
  Text: Flat;
  Result: CompileResult;
  Digits, Report: string;
  Len: Integer;
begin
  Len := FlattenText(Text);
  IdeHost.Compile('P.PAS', @Text, Len, Result);
  if Result.Code = 0 then
  begin
    Say('Compiled');
    Compile := True;
  end
  else
  begin
    Row := Result.Line;
    Col := Result.Col;
    Str(Result.Code, Digits);
    Report := 'Error ' + Digits + ' in ' + Result.FileName;
    Str(Result.Line, Digits);
    Report := Report + ' ' + Digits;
    Str(Result.Col, Digits);
    Say(Report + ':' + Digits);
    Compile := False;
  end;
end;

procedure RunProgram;
var
  Code: Integer;
  Digits: string;
begin
  if not Compile then Exit;
  Code := Run;
  Str(Code, Digits);
  Say('Exit code ' + Digits);
end;

{ Runs whatever was compiled last, without compiling first. }
procedure RunLastModule;
var
  Digits: string;
begin
  Str(Run, Digits);
  Say('Run ' + Digits);
end;

{ The key after ^K; its letter picks the command. }
procedure BlockCommand;
var
  Key: Word;
begin
  Key := GetKey;
  case UpCase(Chr(Lo(Key))) of
    'B', #2: BlockStart := Row;
    'K', #11: BlockEnd := Row;
  else
    Say('Unknown ^K command');
  end;
end;

procedure Edit;
var
  Key: Word;
begin
  Count := 1;
  Row := 1;
  Col := 1;
  BlockStart := 0;
  BlockEnd := 0;
  Log := '';
  Say('Ready');
  repeat
    Paint;
    Key := GetKey;
    case Key of
      KeyEnter: NewLine;
      KeyBackspace: Backspace;
      KeyCtrlK: BlockCommand;
      KeyCtrlY: DeleteLine;
      KeyUp: if Row > 1 then Dec(Row);
      KeyDown: if Row < Count then Inc(Row);
      KeyLeft: if Col > 1 then Dec(Col);
      KeyRight: if Col <= Length(Lines[Row]) then Inc(Col);
      KeyF9: if Compile then;
      KeyCtrlF9: RunProgram;
      KeyF10: RunLastModule;
    else
      if (Key >= 32) and (Key < 127) then Insert(Chr(Key));
    end;
    if Col > Length(Lines[Row]) + 1 then Col := Length(Lines[Row]) + 1;
  until Key = KeyAltX;
end;

end.
