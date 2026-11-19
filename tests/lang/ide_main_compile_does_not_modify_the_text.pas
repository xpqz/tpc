program ide_main_compile_does_not_modify_the_text;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('abc');
  RunIde;
  Report('');
end.
