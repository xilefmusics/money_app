package main

import (
	"io/ioutil"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"xilefmusics.de/money-app/helper"
	"xilefmusics.de/money-app/import_transactions"
	"xilefmusics.de/money-app/transaction"
)

func GetTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	yearFilter := helper.ParseNumber(gc.DefaultQuery("year", "*"), -1, false)
	monthFilter := helper.ParseNumber(gc.DefaultQuery("month", "*"), -1, false)
	podFilter := gc.DefaultQuery("pod", "*")
	debtFilter := gc.DefaultQuery("debt", "*")
	budgetFilter := gc.DefaultQuery("budget", "*")
	inbudgetFilter := gc.DefaultQuery("inbudget", "*")
	typeFilter := gc.DefaultQuery("type", "*")
	idFilter := gc.DefaultQuery("id", "*")

	unfilteredTransactions, err := globalData.GetTransactions(user)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions := transaction.Filter(unfilteredTransactions, yearFilter, monthFilter, podFilter, debtFilter, budgetFilter, inbudgetFilter, typeFilter, idFilter)

	gc.IndentedJSON(http.StatusOK, transactions)
}

func PostTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	body, err := ioutil.ReadAll(gc.Request.Body)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions, err := import_transactions.Import(string(body))
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	newTransactions, err := globalData.AddTransactions(user, transactions, true)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	gc.IndentedJSON(http.StatusOK, newTransactions)
}

func UpdateTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	body, err := ioutil.ReadAll(gc.Request.Body)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions, err := import_transactions.Import(string(body))
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	updated, err := globalData.UpdateTransactions(user, transactions, true)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	gc.IndentedJSON(http.StatusOK, updated)
}

func DeleteTransactions(gc *gin.Context) {
	user, err := helper.GC2User(gc)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	body, err := ioutil.ReadAll(gc.Request.Body)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	transactions, err := import_transactions.Import(string(body))
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	deletedTransactions, err := globalData.DeleteTransactions(user, transactions, true)
	if err != nil {
		log.Printf("ERROR: %s\n", err.Error())
		gc.String(http.StatusInternalServerError, "501 Internal Server Error")
		return
	}

	gc.IndentedJSON(http.StatusOK, deletedTransactions)
}
