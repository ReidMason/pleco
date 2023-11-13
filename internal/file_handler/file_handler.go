package filehandler

import (
	"io/fs"
	"path/filepath"
	"sort"
	"strings"
)

func walk(s string, d fs.DirEntry, err error) error {
	if err != nil {
		return err
	}
	// if !d.IsDir() {
	// 	println(s)
	// }
	return nil
}

func GetFiles(dirpath string) []string {
	files := make([]string, 0)
	filepath.WalkDir(dirpath, func(s string, d fs.DirEntry, err error) error {
		files = append(files, s)
		return walk(s, d, err)
	})

	return files
}

type FilesSummary struct {
	Filecount   int
	CommonTypes FileTypeCountList
}

type FileTypeCount struct {
	Filetype string
	Count    int
}

type FileTypeCountList []FileTypeCount

func (p FileTypeCountList) Len() int           { return len(p) }
func (p FileTypeCountList) Less(i, j int) bool { return p[i].Count < p[j].Count }
func (p FileTypeCountList) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }

func GetFilesSummary(paths []string) FilesSummary {
	counts := make(map[string]int)
	for _, path := range paths {
		file := filepath.Base(path)
		last := file[strings.LastIndex(file, ".")+1:]
		counts[last] += 1
	}

	maxResults := 5
	keys := make(FileTypeCountList, 0, len(counts))
	for k, v := range counts {
		keys = append(keys, FileTypeCount{k, v})
	}
	sort.Sort(sort.Reverse(keys))

	return FilesSummary{
		Filecount:   len(paths),
		CommonTypes: keys[:maxResults],
	}
}
