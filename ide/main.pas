{ Main: the editor loop that ties the IDE modules together.

  One pass of the loop paints the window (View), draws the menu bar and the file name over it
  (Menu, Main), waits for one command (Keys) and carries it out. Keys edits the Buffer itself;
  what it returns is the commands that need another module: the menu (Menu), the files
  (Files) and the host (IdeHost).

  Compile (F9) hands the text in the Buffer to the host as the program CurrentName. The host
  answers with an error number and a position, not a message, so the texts come from Errors
  (a number without a text is shown as "Error N"). When the error is in the text being edited
  the cursor jumps to it; an error in a unit or include file is only named, because that
  file is not in the Buffer.

  Run (Ctrl-F9) always compiles first: a compile is quick and a module that is not made from
  the text on the screen is never run. The host clears the screen for the program, which
  leaves its picture there. Like Turbo Pascal's user screen, Main keeps that picture until a
  key is pressed and then repaints the IDE.

  RunIde returns when the user quits; the IDE program then ends. }
unit Main;

interface

{ Edits the text in the Buffer until the user quits. The caller may fill the Buffer first;
  Files.ResetFiles makes that text the unmodified, unnamed file. }
procedure RunIde;

implementation

uses Crt, Buffer, View, Keys, Menu, Files, IdeHost, Errors;

const
  ScreenWidth = 80;
  TitleRow = 1;
  PromptRow = 25;

function NumberText(Value: LongInt): string;
var
  Digits: string;
begin
  Str(Value, Digits);
  NumberText := Digits;
end;

{ ---- The file name ---- }

{ Writes the current file name at the right end of the menu bar. The cursor stays where it
  was. }
procedure DrawTitle;
var
  Title: string;
  SavedX, SavedY: Byte;
begin
  SavedX := WhereX;
  SavedY := WhereY;
  Title := ' ' + CurrentName + ' ';
  TextAttr := BarAttr;
  GotoXY(ScreenWidth - Length(Title) + 1, TitleRow);
  Write(Title);
  GotoXY(SavedX, SavedY);
end;

{ ---- Compile ---- }

{ The part of the status message after the number, or nothing for an unknown number. }
function MessageOf(Code: Integer): string;
begin
  if ErrorText(Code) = '' then
    MessageOf := ''
  else
    MessageOf := ': ' + ErrorText(Code);
end;

{ Says on the status line which error the host found. An error in the text being edited also
  moves the cursor to it; one in another file is named with its position instead. }
procedure ShowError(var Result: CompileResult);
var
  Text: string;
begin
  Text := 'Error ' + NumberText(Result.Code) + MessageOf(Result.Code);
  if Result.FileName = CurrentName then
    SetCursor(PosOf(Result.Line, Result.Col))
  else
    Text := Result.FileName + ' ' + NumberText(Result.Line) + ':' +
      NumberText(Result.Col) + ' ' + Text;
  SetMessage(Text);
end;

{ Compiles the Buffer as the program CurrentName; True when it compiled. }
function CompileBuffer: Boolean;
var
  Result: CompileResult;
begin
  IdeHost.Compile(CurrentName, TextPointer, TextLength, Result);
  if Result.Code = 0 then
    SetMessage('Compiled')
  else
    ShowError(Result);
  CompileBuffer := Result.Code = 0;
end;

{ ---- Run ---- }

{ The program's picture is on the screen. Tells how it ended, waits for a key and clears the
  screen, so that the next Paint leaves nothing of the program behind. }
procedure KeepUserScreen(ExitCode: Integer);
var
  Key: Char;
begin
  TextAttr := BarAttr;
  GotoXY(1, PromptRow);
  Write('Exit code ', ExitCode, ' - press a key');
  ClrEol;
  Key := ReadKey;
  if Key = #0 then
    Key := ReadKey;
  ClearScreen;
end;

procedure RunProgram;
begin
  if CompileBuffer then
  begin
    KeepUserScreen(IdeHost.Run);
    SetMessage('');
  end;
end;

{ ---- The loop ---- }

{ A new text has no block and starts in Insert mode; a cancelled New or Open changed nothing,
  so the editing state stays. }
procedure ForgetStateOfReplacedText;
begin
  if TextReplaced then
    ResetKeys;
end;

{ Carries out one command. True when the user chose to quit. }
function Execute(Command: Integer): Boolean;
begin
  Execute := False;
  case Command of
    CmdNew:
      begin
        NewFile;
        ForgetStateOfReplacedText;
      end;
    CmdOpen:
      begin
        OpenFile;
        ForgetStateOfReplacedText;
      end;
    CmdSave:
      if SaveFile then;
    CmdCompile:
      if CompileBuffer then;
    CmdRun:
      RunProgram;
    CmdQuit:
      Execute := SaveIfModified;
  end;
end;

procedure RunIde;
var
  Command: Integer;
  Quit: Boolean;
begin
  View.Reset;
  ResetKeys;
  Quit := False;
  repeat
    Paint;
    DrawBar;
    DrawTitle;
    Command := ReadCommand;
    if Command = CmdMenu then
      Command := Menu.Run(MenuLetter);
    Quit := Execute(Command);
  until Quit;
end;

end.
