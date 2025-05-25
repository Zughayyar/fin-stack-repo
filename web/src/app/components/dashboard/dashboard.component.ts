import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, ActivatedRoute } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { ExpenseService, Expense } from '../../services/expense.service';
import { IncomeService, Income } from '../../services/income.service';
import { User, UserService } from '../../services/user.service';
import { AddIncomeComponent } from '../add-income/add-income.component';
import { AddExpenseComponent } from '../add-expense/add-expense.component';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule, AddIncomeComponent, AddExpenseComponent],
  templateUrl: './dashboard.component.html'
})
export class DashboardComponent implements OnInit {
  currentUser: User | null = null;
  expenses: Expense[] = [];
  incomes: Income[] = [];
  selectedMonth: Date = new Date();
  totalExpenses: number = 0;
  totalIncome: number = 0;
  balance: number = 0;

  constructor(
    private expenseService: ExpenseService,
    private incomeService: IncomeService,
    private userService: UserService,
    private route: ActivatedRoute
  ) {}

  ngOnInit() {
    this.route.params.subscribe(params => {
      const userId = params['userId'];
      console.log('Loading user data for ID from route:', userId);
      if (userId) {
        this.loadUserData(userId);
      } else {
        console.error('No user ID found in route parameters');
      }
    });
  }

  loadUserData(userId: string) {
    this.userService.getUserById(userId).subscribe({
      next: (user) => {
        console.log('User data loaded:', user);
        this.currentUser = user;
        this.loadExpenses(userId);
        this.loadIncomes(userId);
      },
      error: (error) => {
        console.error('Error loading user:', error);
      }
    });
  }

  loadExpenses(userId: string) {
    this.expenseService.getExpenses({ user_id: userId }).subscribe({
      next: (expenses) => {
        this.expenses = expenses;
        this.calculateTotals();
      },
      error: (error) => {
        console.error('Error loading expenses:', error);
      }
    });
  }

  loadIncomes(userId: string) {
    this.incomeService.getIncomes({ user_id: userId }).subscribe({
      next: (incomes) => {
        this.incomes = incomes;
        this.calculateTotals();
      },
      error: (error) => {
        console.error('Error loading incomes:', error);
      }
    });
  }

  calculateTotals() {
    this.totalExpenses = this.expenses.reduce((sum, expense) => sum + Number(expense.amount), 0);
    this.totalIncome = this.incomes.reduce((sum, income) => sum + Number(income.amount), 0);
    this.balance = this.totalIncome - this.totalExpenses;
  }

  changeMonth(delta: number) {
    this.selectedMonth = new Date(
      this.selectedMonth.getFullYear(),
      this.selectedMonth.getMonth() + delta,
      1
    );
    // You might want to reload the data for the new month here
  }

  onIncomeAdded() {
    if (this.currentUser) {
      this.loadIncomes(this.currentUser.id);
    }
  }

  onExpenseAdded() {
    if (this.currentUser) {
      this.loadExpenses(this.currentUser.id);
    }
  }
} 