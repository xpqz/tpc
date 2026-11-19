program ide_main_f9_error_says_what_and_jumps;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('begin~  x := ERROR + 1;~end.');
  RunIde;

end.
