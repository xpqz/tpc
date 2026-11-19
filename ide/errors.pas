{ Errors: the message of each compile error number.

  The host's compile answer is a number, not a text (decision 42), so the IDE keeps the texts.
  The table mirrors TABLE in compiler/src/diag.rs, number for number and text for text; a test
  (tests/ide/errors_test.mjs) fails when the two differ. A number that is not in the table has
  no text. }
unit Errors;

interface

{ The message of error Code, or an empty string when the compiler has none. }
function ErrorText(Code: Integer): string;

implementation

type
  ErrorEntry = record
    Code: Integer;
    Text: string[46];
  end;

const
  EntryCount = 75;
  Entries: array[1..EntryCount] of ErrorEntry = (
    (Code: 2; Text: 'Identifier expected'),
    (Code: 3; Text: 'Unknown identifier'),
    (Code: 4; Text: 'Duplicate identifier'),
    (Code: 5; Text: 'Syntax error'),
    (Code: 6; Text: 'Error in real constant'),
    (Code: 7; Text: 'Error in integer constant'),
    (Code: 8; Text: 'String constant exceeds line'),
    (Code: 9; Text: 'Too many nested files'),
    (Code: 10; Text: 'Unexpected end of file'),
    (Code: 12; Text: 'Type identifier expected'),
    (Code: 15; Text: 'File not found'),
    (Code: 17; Text: 'Invalid compiler directive'),
    (Code: 19; Text: 'Undefined type in pointer definition'),
    (Code: 20; Text: 'Variable identifier expected'),
    (Code: 21; Text: 'Error in type'),
    (Code: 22; Text: 'Structure too large'),
    (Code: 23; Text: 'Set base type out of range'),
    (Code: 25; Text: 'Invalid string length'),
    (Code: 26; Text: 'Type mismatch'),
    (Code: 28; Text: 'Lower bound greater than upper bound'),
    (Code: 29; Text: 'Ordinal type expected'),
    (Code: 30; Text: 'Integer constant expected'),
    (Code: 31; Text: 'Constant expected'),
    (Code: 33; Text: 'Type identifier expected'),
    (Code: 34; Text: 'Invalid function result type'),
    (Code: 36; Text: 'BEGIN expected'),
    (Code: 37; Text: 'END expected'),
    (Code: 38; Text: 'Integer expression expected'),
    (Code: 39; Text: 'Ordinal expression expected'),
    (Code: 40; Text: 'Boolean expression expected'),
    (Code: 41; Text: 'Operand types do not match operator'),
    (Code: 42; Text: 'Error in expression'),
    (Code: 43; Text: 'Illegal assignment'),
    (Code: 44; Text: 'Field identifier expected'),
    (Code: 50; Text: 'DO expected'),
    (Code: 54; Text: 'OF expected'),
    (Code: 55; Text: 'INTERFACE expected'),
    (Code: 57; Text: 'THEN expected'),
    (Code: 58; Text: 'TO or DOWNTO expected'),
    (Code: 59; Text: 'Undefined forward'),
    (Code: 62; Text: 'Division by zero'),
    (Code: 64; Text: 'Cannot Read or Write variables of this type'),
    (Code: 68; Text: 'Circular unit reference'),
    (Code: 69; Text: 'Unit name mismatch'),
    (Code: 73; Text: 'IMPLEMENTATION expected'),
    (Code: 74; Text: 'Constant and case types do not match'),
    (Code: 75; Text: 'Record variable expected'),
    (Code: 76; Text: 'Constant out of range'),
    (Code: 77; Text: 'File variable expected'),
    (Code: 84; Text: 'UNIT expected'),
    (Code: 85; Text: '";" expected'),
    (Code: 86; Text: '":" expected'),
    (Code: 87; Text: '"," expected'),
    (Code: 88; Text: '"(" expected'),
    (Code: 89; Text: '")" expected'),
    (Code: 90; Text: '"=" expected'),
    (Code: 91; Text: '":=" expected'),
    (Code: 93; Text: '"]" or ".)" expected'),
    (Code: 94; Text: '"." expected'),
    (Code: 95; Text: '".." expected'),
    (Code: 96; Text: 'Too many variables'),
    (Code: 97; Text: 'Invalid FOR control variable'),
    (Code: 98; Text: 'Integer variable expected'),
    (Code: 99; Text: 'Files and procedure types are not allowed here'),
    (Code: 100; Text: 'String length mismatch'),
    (Code: 101; Text: 'Invalid ordering of fields'),
    (Code: 112; Text: 'CASE constant out of range'),
    (Code: 113; Text: 'Error in statement'),
    (Code: 118; Text: 'Include files are not allowed here'),
    (Code: 121; Text: 'Invalid qualifier'),
    (Code: 126; Text: 'Files must be var parameters'),
    (Code: 128; Text: 'Misplaced conditional directive'),
    (Code: 129; Text: 'ENDIF directive missing'),
    (Code: 133; Text: 'Cannot evaluate this expression'),
    (Code: 200; Text: 'Feature not supported')
  );

function ErrorText(Code: Integer): string;
var
  Index: Integer;
begin
  ErrorText := '';
  for Index := 1 to EntryCount do
    if Entries[Index].Code = Code then
      ErrorText := Entries[Index].Text;
end;

end.
