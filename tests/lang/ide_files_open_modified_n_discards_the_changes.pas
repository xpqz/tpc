program ide_files_open_modified_n_discards_the_changes;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('mine');
  EditLoop;
  Report('GOOD.TXT');
end.
