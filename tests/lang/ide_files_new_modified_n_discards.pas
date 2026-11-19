program ide_files_new_modified_n_discards;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('NONAME.PAS');
end.
