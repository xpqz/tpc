program ide_main_f9_good_says_compiled;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('program p;~begin~end.');
  RunIde;

end.
