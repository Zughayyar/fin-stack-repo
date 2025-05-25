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
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateExpenseRequest {
  item_name: string;
  amount: string;
  date: string;
  description?: string;
}

@Injectable({
  providedIn: 'root'
})
export class ExpenseService {
  private apiUrl = environment.apiUrl;

  constructor(private http: HttpClient) {}

  getExpenses(userId: string): Observable<Expense[]> {
    return this.http.get<Expense[]>(`${this.apiUrl}/users/${userId}/expenses`);
  }

  getExpenseById(userId: string, expenseId: string): Observable<Expense> {
    return this.http.get<Expense>(`${this.apiUrl}/users/${userId}/expenses/${expenseId}`);
  }

  createExpense(userId: string, expense: CreateExpenseRequest): Observable<Expense> {
    return this.http.post<Expense>(`${this.apiUrl}/users/${userId}/expenses`, expense);
  }

  updateExpense(userId: string, expenseId: string, expense: Partial<CreateExpenseRequest>): Observable<Expense> {
    return this.http.patch<Expense>(`${this.apiUrl}/users/${userId}/expenses/${expenseId}`, expense);
  }

  deleteExpense(userId: string, expenseId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/users/${userId}/expenses/${expenseId}`);
  }
} 