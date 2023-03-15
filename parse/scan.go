package parse

import (
	"bufio"
	"fmt"
	"os"
)

type Scanner interface {
	Next() (string, bool)
}
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

func NewFileScanner(path string) (*FileScanner, error) {
	var fileHandles []*fileHandle
	firstFileHandle, err := NewFileHandle(path)
	if err != nil {
		return nil, err
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
				eventChan <- NewLineEvent(line, fileHandle.Lineno)
			} else {
				fileHandles = fileHandles[:len(fileHandles)-1]
				if len(fileHandles) > 0 {
					eventChan <- NewFileEvent(fileHandles[len(fileHandles)-1].Filename)
				}
			}
		}
	}()
	return &FileScanner{fileHandles, eventChan}, nil
}

func (s *FileScanner) Next() (ScanEvent, bool) {
	event, ok := <-s.eventChan
	return event, ok
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
