program ide_main_f2_saves_again_without_asking;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('x');
  RunIde;
  Report('OUT.PAS');
end.
