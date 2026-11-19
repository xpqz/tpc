program ide_files_save_then_save_again_after_more_editing;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('S.PAS');
end.
