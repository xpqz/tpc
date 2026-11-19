program ide_files_save_unnamed_backspace_edits_the_name;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('ABC.PAS');
end.
