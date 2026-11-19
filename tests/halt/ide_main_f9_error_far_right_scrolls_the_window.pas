program ide_main_f9_error_far_right_scrolls_the_window;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

procedure Put(S: string);
var I: Integer;
begin
  for I := 1 to Length(S) do InsertChar(S[I]);
end;

procedure Fill(Count: Integer);
var N: Integer; S: string;
begin
  for N := 1 to Count do
  begin
    Str(N, S);
    Put('line ' + S);
    InsertChar(LineBreak);
  end;
end;

begin
  Start('');
  Put('x := 1; { 0123456789 0123456789 0123456789 0123456789 0123456789 0123456789 0123456789 0123456789 } ERROR');
  SetCursor(0);
  ResetFiles;
  RunIde;
end.
