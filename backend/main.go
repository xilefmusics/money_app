package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"

	"xilefmusics.de/money-app/config"
	"xilefmusics.de/money-app/data"
)

var globalData data.Data

func main() {
	c := config.Load()

	d, err := data.New(c.DataPath)
	if err != nil {
		log.Fatal(err.Error())
	}
	globalData = d

	router := gin.New()
	router.GET("/transactions", GetTransactions)
	router.POST("/transactions", PostTransactions)
	router.PUT("/transactions", UpdateTransactions)
	router.GET("/lint", Lint)
	router.GET("/reindex", Reindex)
	router.GET("/pods", GetPods)
	router.GET("/debts", GetDebts)
	router.GET("/budgets", GetBudgets)
	router.GET("/inbudgets", GetInbudgets)
	router.GET("/tags", GetTags)
	router.GET("/history/:kind", GetHistory)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", c.Port), router))
}
