package helper

import (
	"mime"
	"strings"
)

func FileName2MimeType(fileName string) string {
	defaultMime := "application/octet-stream"

	split := strings.Split(fileName, ".")
	if len(split) < 2 {
		return defaultMime
	}
	ext := "." + split[len(split)-1]

	return mime.TypeByExtension(ext)
}

func MimeType2Extension(mimeType string) string {
	defaultExtension := ".bin"

	extensions, err := mime.ExtensionsByType(mimeType)
	if err != nil || len(extensions) == 0 {
		return defaultExtension
	}

	if extensions[0] == ".jpe" {
		return ".jpeg"
	}

	return extensions[0]
}
