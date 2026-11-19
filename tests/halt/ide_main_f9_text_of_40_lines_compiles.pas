program ide_main_f9_text_of_40_lines_compiles;
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
  Fill(40);
  SetCursor(0);
  ResetFiles;
  RunIde;
end.
