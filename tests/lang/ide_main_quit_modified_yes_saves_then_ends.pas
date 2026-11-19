program ide_main_quit_modified_yes_saves_then_ends;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('abc');
  RunIde;
  Report('OUT.PAS');
end.
