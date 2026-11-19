program scroll_keeps_cursor_row_while_moving_inside_the_window;
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
  NumberedLines(100);
  SetCursor(PosOf(50, 1));
  Paint;
  for N := 1 to 10 do
  begin
    MoveUp;
    Paint;
  end;
  MoveDown;
  MoveDown;
  Paint;
end.
