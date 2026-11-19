program ide_files_save_unnamed_escape_and_empty_name_cancel;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('keep');
  EditLoop;
  Report('NONAME.PAS');
end.
