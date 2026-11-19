program ide_files_save_unnamed_asks_for_a_name_and_adds_pas;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('one~two');
  EditLoop;
  Report('OUT.PAS');
end.
