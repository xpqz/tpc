program tabs_expand_to_multiples_of_eight;
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
  PutLine(#9 + 'tab first');
  PutLine('ab' + #9 + 'two letters');
  PutLine('1234567' + #9 + 'seven');
  PutLine('12345678' + #9 + 'eight');
  PutLine(#9 + #9 + 'two tabs');
  SetCursor(PosOf(2, 4));
  Paint;
end.
