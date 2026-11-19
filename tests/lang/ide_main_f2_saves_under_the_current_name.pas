program ide_main_f2_saves_under_the_current_name;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('x~y');
  RunIde;
  Report('OUT.PAS');
end.
