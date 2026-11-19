program ide_files_save_marks_the_text_unmodified_so_open_and_new_do_not_ask;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('z');
  EditLoop;
  Report('S.PAS');
end.
