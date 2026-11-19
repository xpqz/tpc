program ide_files_quit_modified_n_leaves;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('');
end.
