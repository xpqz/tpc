program typing_a_screenful_with_the_view_painting_each_line;
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
var N: Integer; S: string;
begin
  Reset;
  for N := 1 to 30 do
  begin
    Str(N, S);
    PutLine('typed ' + S);
    Paint;
  end;
  Put('now typing');
  Paint;
end.
