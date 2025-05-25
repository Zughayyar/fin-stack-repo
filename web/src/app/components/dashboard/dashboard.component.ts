import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute } from '@angular/router';
import { UserService, User } from '../../services/user.service';
import { ExpenseService, Expense } from '../../services/expense.service';
import { IncomeService, Income } from '../../services/income.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  standalone: true,
  imports: [CommonModule]
})
export class DashboardComponent implements OnInit {
  currentUser?: User;
  expenses: Expense[] = [];
  incomes: Income[] = [];
  selectedMonth: Date = new Date();
  totalIncome: number = 0;
  totalExpenses: number = 0;
  balance: number = 0;

  constructor(
    private route: ActivatedRoute,
    private userService: UserService,
    private expenseService: ExpenseService,
    private incomeService: IncomeService
  ) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const userId = params['userId'];
      if (userId) {
        this.loadUserData(userId);
      }
    });
  }

  loadUserData(userId: string): void {
    // Load user details
    this.userService.getUserById(userId).subscribe(user => {
      this.currentUser = user;
    });

    // Load expenses
    this.expenseService.getExpenses(userId).subscribe(expenses => {
      this.expenses = this.filterByMonth(expenses);
      this.calculateTotals();
    });

    // Load incomes
    this.incomeService.getIncomes(userId).subscribe(incomes => {
      this.incomes = this.filterByMonth(incomes);
      this.calculateTotals();
    });
  }

  filterByMonth<T extends { date: string }>(items: T[]): T[] {
    const startOfMonth = new Date(this.selectedMonth.getFullYear(), this.selectedMonth.getMonth(), 1);
    const endOfMonth = new Date(this.selectedMonth.getFullYear(), this.selectedMonth.getMonth() + 1, 0);
    
    return items.filter(item => {
      const itemDate = new Date(item.date);
      return itemDate >= startOfMonth && itemDate <= endOfMonth;
    });
  }

  calculateTotals(): void {
    this.totalIncome = this.incomes.reduce((sum, income) => sum + parseFloat(income.amount), 0);
    this.totalExpenses = this.expenses.reduce((sum, expense) => sum + parseFloat(expense.amount), 0);
    this.balance = this.totalIncome - this.totalExpenses;
  }

  changeMonth(months: number): void {
    this.selectedMonth = new Date(
      this.selectedMonth.getFullYear(),
      this.selectedMonth.getMonth() + months,
      1
    );
    if (this.currentUser) {
      this.loadUserData(this.currentUser.id);
    }
  }
} 