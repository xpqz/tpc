program ide_main_error_fixed_then_compile_runs;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('begin~  ERROR~end.');
  RunIde;

end.
