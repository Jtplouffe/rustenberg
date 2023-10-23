package rustenberg

import (
	"os"
	"path/filepath"
	"strings"
)

type File interface {
	name() (string, error)
	content() ([]byte, error)
}

type MemoryFile struct {
	n string
	c []byte
}

func NewFileFromMemory(name string, content []byte) *MemoryFile {
	return &MemoryFile{
		n: name,
		c: content,
	}
}

func (file *MemoryFile) name() (string, error) {
	return file.n, nil
}

func (file *MemoryFile) content() ([]byte, error) {
	return file.c, nil
}

type PathFile struct {
	path string
}

func NewFileFromPath(path string) *PathFile {
	return &PathFile{
		path: path,
	}
}

func (file *PathFile) name() (string, error) {
	base := filepath.Base(file.path)
	ext := filepath.Ext(file.path)
	return strings.TrimSuffix(base, ext), nil
}

func (file *PathFile) content() ([]byte, error) {
	return os.ReadFile(file.path)
}
