program horizontal_scroll_shows_each_row_with_the_same_offset;
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
var N: Integer;
begin
  Reset;
  for N := 1 to 5 do
  begin
    Dashes(N * 40);
    InsertChar(LineBreak);
  end;
  SetCursor(PosOf(5, 130));
  Paint;
end.
