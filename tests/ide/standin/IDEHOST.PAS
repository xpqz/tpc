{ Stand-in for the IDE host unit, for the real Turbo Pascal 5.0 (the oracle). The real
  IdeHost is embedded in the compiler and calls the host; this one has the same interface and
  canned behaviour, which the runner's canned compiler (tests/ide/canned.mjs) repeats:
    Compile fails with error 3 at the first line that contains the word ERROR (column of its
    first letter, the file named by Name); every other text compiles.
    Run returns the number of lines of the last compiled text, modulo 256, and draws nothing;
    it returns -2 when the last Compile failed or there was none. }
unit IdeHost;

interface

type
  CompileResult = record
    Code: Integer;
    Line, Col: LongInt;
    FileName: string[12];
  end;

procedure Compile(Name: string; Text: Pointer; Len: LongInt; var Result: CompileResult);
function Run: Integer;

implementation

type
  Bytes = array[0..65520] of Char;
  BytesPtr = ^Bytes;

var
  LinesCompiled: LongInt;
  Compiled: Boolean;

function StartsWord(var T: Bytes; At, Len: LongInt): Boolean;
begin
  StartsWord := (At + 5 <= Len) and (T[At] = 'E') and (T[At + 1] = 'R') and
    (T[At + 2] = 'R') and (T[At + 3] = 'O') and (T[At + 4] = 'R');
end;

procedure Compile(Name: string; Text: Pointer; Len: LongInt; var Result: CompileResult);
var
  T: BytesPtr;
  I, Line, LineStart: LongInt;
begin
  T := Text;
  Compiled := False;
  Result.Code := 0;
  Result.Line := 0;
  Result.Col := 0;
  Result.FileName := '';
  Line := 1;
  LineStart := 0;
  for I := 0 to Len - 1 do
  begin
    if (Result.Code = 0) and StartsWord(T^, I, Len) then
    begin
      Result.Code := 3;
      Result.Line := Line;
      Result.Col := I - LineStart + 1;
      Result.FileName := Name;
    end;
    if T^[I] = #10 then
    begin
      Inc(Line);
      LineStart := I + 1;
    end;
  end;
  LinesCompiled := Line - 1;
  Compiled := Result.Code = 0;
end;

function Run: Integer;
begin
  if Compiled then Run := LinesCompiled mod 256 else Run := -2;
end;

end.
