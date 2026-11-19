program ide_main_f9_error_then_fixed_says_compiled;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('x~ERROR~y');
  RunIde;

end.
