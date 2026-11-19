{ The IdeHost unit: how the IDE asks its host to compile and run a program (decision 42).
  The IDE cannot compile or run anything itself, so both routines call the host:
    ide_compile(name, text, length, result) compiles the text as the main program, using the
      source files of the host's file system for units and include files, and fills `result`;
      the compiled module stays in the host.
    ide_run() runs the module of the last successful compile and returns when the program has
      ended or been stopped.
  The host reads and writes the memory of the IDE through the pointers, so CompileResult must
  keep its layout: 2 + 4 + 4 + 13 = 23 bytes, without padding. }
unit IdeHost;

interface

type
  CompileResult = record
    Code: Integer;
    Line, Col: LongInt;
    FileName: string[12];
  end;

{ Compiles Text (Len bytes, contiguous) as the program Name. Code is 0 on success; otherwise
  it is the error number and Line, Col and FileName say where (FileName is Name for errors in
  the program itself, else the unit or include file). }
procedure Compile(Name: string; Text: Pointer; Len: LongInt; var Result: CompileResult);

{ Runs the last module: its exit code (0..255), -1 when the user stopped it, -2 when the last
  Compile failed or there was none. }
function Run: Integer;

implementation

procedure HostCompile(Name, Text: Pointer; Len: LongInt; Result: Pointer); external 'ide_compile';
function HostRun: LongInt; external 'ide_run';

procedure Compile(Name: string; Text: Pointer; Len: LongInt; var Result: CompileResult);
begin
  HostCompile(@Name, Text, Len, @Result);
end;

function Run: Integer;
begin
  Run := HostRun;
end;

end.
