{ A toy IDE shell around the IDE host unit: the text buffer is filled by typing or by File
  Open (F3, then a typed name), saved with F2, compiled with F9, compiled and run with
  Ctrl-F9, run without compiling with F10 and left with Alt-X. The status line reports each
  result. After a run it shows the exit code and waits for a key, as the IDE does, then
  repaints. }
program IdeHostToy;

uses Crt, IdeHost;

const
  MaxText = 4000;
  LogLines = 12;
  KeyEnter = $000D;
  KeyBackspace = $0008;
  KeyF2 = $3C00;
  KeyF3 = $3D00;
  KeyF9 = $4300;
  KeyF10 = $4400;
  KeyCtrlF9 = $6600;
  KeyAltX = $2D00;

var
  Buffer: array[0..MaxText] of Char;
  Len: Integer;
  FileName, Status: string;
  Log: array[1..LogLines] of string;
  LogCount: Integer;

function GetKey: Word;
var
  C: Char;
begin
  C := ReadKey;
  if C = #0 then GetKey := Ord(ReadKey) shl 8 else GetKey := Ord(C);
end;

function Number(N: LongInt): string;
var
  Digits: string;
begin
  Str(N, Digits);
  Number := Digits;
end;

{ Sets the status line and keeps every message, oldest first, in the log below it. }
procedure Say(Message: string);
var
  I: Integer;
begin
  Status := Message;
  if LogCount = LogLines then
  begin
    for I := 1 to LogLines - 1 do Log[I] := Log[I + 1];
    Dec(LogCount);
  end;
  Inc(LogCount);
  Log[LogCount] := Message;
end;

procedure Paint;
var
  I: Integer;
begin
  ClrScr;
  GotoXY(1, 1);
  Write('File: ', FileName);
  GotoXY(1, 3);
  Write(Status);
  for I := 1 to LogCount do
  begin
    GotoXY(1, 4 + I);
    Write(Log[I]);
  end;
end;

procedure Append(Ch: Char);
begin
  if Len < MaxText then
  begin
    Buffer[Len] := Ch;
    Inc(Len);
  end;
end;

{ Reads a file name typed after F3 or the like, upper-cased, up to Enter. }
function ReadName(Prompt: string): string;
var
  Name: string;
  Key: Word;
begin
  Name := '';
  repeat
    Status := Prompt + Name;
    Paint;
    Key := GetKey;
    if (Key >= 32) and (Key < 127) then Name := Name + UpCase(Chr(Key))
    else if (Key = KeyBackspace) and (Length(Name) > 0) then Dec(Name[0]);
  until Key = KeyEnter;
  ReadName := Name;
end;

procedure OpenFile;
var
  Name: string;
  F: Text;
  C: Char;
begin
  Name := ReadName('Open: ');
  Assign(F, Name);
  {$I-}
  Reset(F);
  {$I+}
  if IOResult <> 0 then
  begin
    Say('Cannot open ' + Name);
    Exit;
  end;
  Len := 0;
  while not Eof(F) do
  begin
    Read(F, C);
    Append(C);
  end;
  Close(F);
  FileName := Name;
  Say('Opened ' + Name);
end;

procedure SaveFile;
var
  F: Text;
  I: Integer;
begin
  Assign(F, FileName);
  Rewrite(F);
  for I := 0 to Len - 1 do Write(F, Buffer[I]);
  Close(F);
  Say('Saved ' + FileName);
end;

function CompileBuffer: Boolean;
var
  Result: CompileResult;
begin
  IdeHost.Compile(FileName, @Buffer, Len, Result);
  if Result.Code = 0 then Say('Compiled')
  else
    Say('Error ' + Number(Result.Code) + ' in ' + Result.FileName + ' ' +
      Number(Result.Line) + ':' + Number(Result.Col));
  CompileBuffer := Result.Code = 0;
end;

{ Runs the last module; the program's screen stays visible until a key is pressed. }
procedure RunModule;
var
  Code: Integer;
  Key: Word;
begin
  Code := Run;
  Say('Exit code ' + Number(Code));
  GotoXY(1, 25);
  Write(Status, ' - press a key');
  Key := GetKey;
end;

var
  Key: Word;

begin
  Len := 0;
  FileName := 'P.PAS';
  LogCount := 0;
  Say('Ready');
  repeat
    Paint;
    Key := GetKey;
    case Key of
      KeyEnter:
        begin
          Append(#13);
          Append(#10);
        end;
      KeyBackspace: if Len > 0 then Dec(Len);
      KeyF2: SaveFile;
      KeyF3: OpenFile;
      KeyF9: if CompileBuffer then;
      KeyCtrlF9: if CompileBuffer then RunModule;
      KeyF10: RunModule;
    else
      if (Key >= 32) and (Key < 127) then Append(Chr(Key));
    end;
  until Key = KeyAltX;
end.
