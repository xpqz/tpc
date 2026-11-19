program ide_files_new_modified_escape_keeps_everything;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc~def');
  EditLoop;
  Report('NONAME.PAS');
end.
