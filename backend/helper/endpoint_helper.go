package helper

import (
	"errors"
	"strconv"

	"github.com/gin-gonic/gin"
)

func GC2User(gc *gin.Context) (string, error) {
	users := gc.Request.Header["X-Remote-User"]
	if len(users) != 1 {
		return "", errors.New("GC2User: Not exactly one user was given")
	}
	return users[0], nil
}

func ParseNumber(str string, defaultValue int, nonNegative bool) int {
	number, err := strconv.ParseInt(str, 10, 32)
	if err != nil {
		number = int64(defaultValue)
	}
	if number < 0 && nonNegative {
		number = int64(defaultValue)
	}
	return int(number)
}
