program ide_files_new_after_new_is_unmodified;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('');
end.
