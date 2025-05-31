import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Expense {
  id: string;
  user_id: string;
  item_name: string;
  amount: string;
  date: string;
  description?: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewExpense {
  user_id: string;
  item_name: string;
  amount: string;
  date: string;
  description?: string | null;
}

export interface UpdateExpense {
  item_name?: string | null;
  amount?: string | null;
  date?: string | null;
  description?: string | null;
  updated_at?: string | null;
}

@Injectable({
  providedIn: 'root'
})
export class ExpenseService {
  private apiUrl = environment.apiUrl;

  constructor(private http: HttpClient) {}

  // GET /api/expenses (all expenses)
  getAllExpenses(): Observable<Expense[]> {
    return this.http.get<Expense[]>(`${this.apiUrl}/api/expenses`);
  }

  // GET /api/expenses/user/{user_id}
  getExpensesByUserId(userId: string): Observable<Expense[]> {
    return this.http.get<Expense[]>(`${this.apiUrl}/api/expenses/${userId}`);
  }

  // POST /api/expenses
  createExpense(expense: NewExpense): Observable<Expense> {
    return this.http.post<Expense>(`${this.apiUrl}/api/expenses`, expense);
  }

  // PUT /api/expenses/{expense_id}
  updateExpense(expenseId: string, update: UpdateExpense): Observable<Expense> {
    return this.http.put<Expense>(`${this.apiUrl}/api/expenses/${expenseId}`, update);
  }

  // DELETE /api/expenses/{expense_id}
  deleteExpense(expenseId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/api/expenses/${expenseId}`);
  }
} 