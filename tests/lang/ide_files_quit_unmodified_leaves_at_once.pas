program ide_files_quit_unmodified_leaves_at_once;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('');
end.
