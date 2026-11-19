program tabs_in_a_long_line_scroll_by_screen_columns;
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
  for N := 1 to 12 do
    Put('a' + #9);
  Dashes(30);
  SetCursor(PosOf(1, 20));
  Paint;
end.
