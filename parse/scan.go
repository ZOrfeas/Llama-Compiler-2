package parse

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type fileHandle struct {
	lineChan chan string
	Filename string
	Lineno   uint
}

func NewFileHandle(path string) (*fileHandle, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	scanner := bufio.NewScanner(file)
	lineChan := make(chan string)
	go func() {
		defer file.Close()
		defer close(lineChan)
		for scanner.Scan() {
			lineChan <- scanner.Text()
		}
	}()
	return &fileHandle{lineChan, path, 1}, nil
}
func (fh *fileHandle) Next() (string, bool) {
	line, ok := <-fh.lineChan
	if ok {
		fh.Lineno++
	}
	return line, ok
}

type FileScanner struct {
	fileHandles []*fileHandle
	eventChan   chan ScanEvent
}

func NewFileScanner(path string) *FileScanner {
	var fileHandles []*fileHandle
	firstFileHandle, err := NewFileHandle(path)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fileHandles = append(fileHandles, firstFileHandle)
	eventChan := make(chan ScanEvent)
	go func() {
		defer close(eventChan)
		eventChan <- NewFileEvent(path)
		for len(fileHandles) > 0 {
			fileHandle := fileHandles[len(fileHandles)-1]
			line, ok := fileHandle.Next()
			if ok {
				// TODO: Handle includes here
				includePath, err := handleInclude(line)
				if err != nil {
					fmt.Fprintln(os.Stderr, err)
					os.Exit(1)
				}
				if includePath != "" {
					includeFileHandle, err := NewFileHandle(includePath)
					if err != nil {
						fmt.Fprintln(os.Stderr, err)
						os.Exit(1)
					}
					fileHandles = append(fileHandles, includeFileHandle)
					eventChan <- NewFileEvent(includePath)
					continue
				}
				eventChan <- NewLineEvent(line, fileHandle.Lineno)
			} else {
				fileHandles = fileHandles[:len(fileHandles)-1]
				if len(fileHandles) > 0 {
					eventChan <- NewFileEvent(fileHandles[len(fileHandles)-1].Filename)
				}
			}
		}
	}()
	return &FileScanner{fileHandles, eventChan}
}

func handleInclude(line string) (string, error) {
	trimmedLine := strings.TrimSpace(line)
	if len(trimmedLine) != 0 && trimmedLine[0] != '#' {
		return "", nil
	}
	if trimmedLine[0] == '#' && !strings.HasPrefix(trimmedLine, "#include") {
		return "", fmt.Errorf("invalid preprocessor directive (reminder: must be #include)")
	}
	includePath := strings.TrimSpace(strings.TrimPrefix(trimmedLine, "#include"))
	if includePath == "" {
		return "", fmt.Errorf("empty include path")
	}
	if includePath[0] != '"' || includePath[len(includePath)-1] != '"' {
		return "", fmt.Errorf("invalid include path (reminder: must be surrounded by double quotes)")
	}
	return includePath[1 : len(includePath)-1], nil
}

func (s *FileScanner) Next() (ScanEvent, bool) {
	event, ok := <-s.eventChan
	return event, ok
}
func (s *FileScanner) Iter() <-chan ScanEvent {
	return s.eventChan
}

type ScanEvent struct {
	text           string
	lineno         uint
	is_file_change bool
}

func NewLineEvent(text string, lineno uint) ScanEvent {
	return ScanEvent{text, lineno, false}
}
func NewFileEvent(text string) ScanEvent {
	return ScanEvent{text, 0, true}
}

func (e ScanEvent) String() string {
	var prefix string
	if e.is_file_change {
		prefix = "file  : "
	} else {
		prefix = "line " + fmt.Sprint(e.lineno) + ": "
	}
	return prefix + e.text
}
func (e ScanEvent) SourceLine() string {
	if e.is_file_change {
		return ""
	}
	return e.text
}
