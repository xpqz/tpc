{ Buffer: Load and Save with text files: CRLF lines, a missing final line break, an empty
  file, a line longer than a string, a missing file, saving and reloading, and the capacity
  limit of a file that just fits and one that does not. }
program LoadAndSaveTextFiles;

uses Buffer;

procedure Show(Title: string);
var
  Pos: LongInt;
begin
  Write(Title, ': len ', TextLength, ' cur ', Cursor, ' lines ', LineCount, ' [');
  for Pos := 0 to TextLength - 1 do
    if Pos < 60 then
      if CharAt(Pos) = LineBreak then Write('|') else Write(CharAt(Pos));
  WriteLn(']');
end;

procedure Type_(S: string);
var
  I: Integer;
begin
  for I := 1 to Length(S) do
    if S[I] = '/' then
      InsertChar(LineBreak)
    else
      InsertChar(S[I]);
end;

{ The bytes of a file as numbers, so the line endings show. }
procedure Dump(Name: string);
var
  F: Text;
  Ch: Char;
begin
  Assign(F, Name);
  Reset(F);
  Write(Name, ':');
  while not Eof(F) do
  begin
    while not Eoln(F) do
    begin
      Read(F, Ch);
      Write(' ', Ord(Ch));
    end;
    if Eof(F) then
      Write(' (no EOL)')
    else
    begin
      ReadLn(F);
      Write(' EOL');
    end;
  end;
  WriteLn;
  Close(F);
end;

{ Writes Lines lines of Width characters, then Extra more characters, into Name. }
procedure WriteFile(Name: string; Lines, Width, Extra: Integer);
var
  F: Text;
  Line: string;
  I: Integer;
begin
  Assign(F, Name);
  Rewrite(F);
  Line := '';
  for I := 1 to Width do
    Line := Line + 'x';
  for I := 1 to Lines do
    WriteLn(F, Line);
  Line := '';
  for I := 1 to Extra do
    Line := Line + 'y';
  Write(F, Line);
  Close(F);
end;

var
  Code: Integer;
begin
  Code := Load('CRLF.TXT');
  WriteLn('load CRLF.TXT = ', Code);
  Show('crlf');
  Code := Load('NOEOL.TXT');
  WriteLn('load NOEOL.TXT = ', Code);
  Show('no final break');
  Code := Load('EMPTY.TXT');
  WriteLn('load EMPTY.TXT = ', Code);
  Show('empty');
  Code := Load('LONG.TXT');
  WriteLn('load LONG.TXT = ', Code);
  WriteLn('long line: len ', TextLength, ' lines ', LineCount, ' line 1 ends at ', LineEndPos(0),
    ' second line starts ', PosOf(2, 1));

  Code := Load('CRLF.TXT');
  SetCursor(3);
  Code := Load('NOSUCH.TXT');
  WriteLn('load NOSUCH.TXT = ', Code);
  Show('unchanged after a failed load');

  Clear;
  Type_('one/two/');
  Code := Save('OUT.TXT');
  WriteLn('save = ', Code);
  Dump('OUT.TXT');
  Type_('three');
  Code := Save('OUT.TXT');
  Dump('OUT.TXT');
  Clear;
  Code := Save('OUT.TXT');
  Dump('OUT.TXT');
  Code := Load('OUT.TXT');
  Show('reloaded empty');
  Clear;
  Type_('/start/ /end/');
  Code := Save('OUT.TXT');
  Dump('OUT.TXT');
  Code := Load('OUT.TXT');
  Show('round trip');

  { 655 lines of 99 characters plus one of 20 fill the buffer to MaxText exactly. }
  WriteFile('FIT.TXT', 655, 99, 20);
  Code := Load('FIT.TXT');
  WriteLn('load FIT.TXT = ', Code, ' len ', TextLength, ' lines ', LineCount);
  InsertChar('z');
  WriteLn('full ', IsFull, ' length after one more insert ', TextLength);
  Code := Save('OUT2.TXT');
  Code := Load('OUT2.TXT');
  WriteLn('save and load again = ', Code, ' len ', TextLength, ' lines ', LineCount);
  WriteLn('last line starts ', PosOf(LineCount, 1), ' char before ', Ord(CharAt(PosOf(LineCount, 1) - 1)));
  WriteFile('BIG.TXT', 655, 99, 21);
  Code := Load('BIG.TXT');
  WriteLn('load BIG.TXT = ', Code);
  Show('too large leaves it empty');
end.
