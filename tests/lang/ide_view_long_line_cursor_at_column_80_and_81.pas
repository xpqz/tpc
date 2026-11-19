program long_line_cursor_at_column_80_and_81;
uses Crt, Buffer, View;

procedure Put(S: string);
var I: Integer;
begin
  for I := 1 to Length(S) do InsertChar(S[I]);
end;

procedure PutLine(S: string);
begin
  Put(S);
  InsertChar(LineBreak);
end;

procedure NumberedLines(Count: Integer);
var N: Integer; S: string;
begin
  for N := 1 to Count do
  begin
    Str(N, S);
    PutLine('line ' + S);
  end;
end;

procedure Dashes(Count: Integer);
var N: Integer;
begin
  for N := 1 to Count do
    InsertChar(Char(Ord('0') + N mod 10));
end;
begin
  Reset;
  Dashes(100);
  InsertChar(LineBreak);
  Dashes(100);
  SetCursor(PosOf(1, 80));
  Paint;
  SetCursor(PosOf(2, 81));
  Paint;
end.
