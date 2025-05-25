import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Income {
  id: string;
  user_id: string;
  source: string;
  amount: string;
  date: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateIncomeRequest {
  source: string;
  amount: string;
  date: string;
  description?: string;
}

@Injectable({
  providedIn: 'root'
})
export class IncomeService {
  private apiUrl = environment.apiUrl;

  constructor(private http: HttpClient) {}

  getIncomes(userId: string): Observable<Income[]> {
    return this.http.get<Income[]>(`${this.apiUrl}/users/${userId}/income`);
  }

  getIncomeById(userId: string, incomeId: string): Observable<Income> {
    return this.http.get<Income>(`${this.apiUrl}/users/${userId}/income/${incomeId}`);
  }

  createIncome(userId: string, income: CreateIncomeRequest): Observable<Income> {
    return this.http.post<Income>(`${this.apiUrl}/users/${userId}/income`, income);
  }

  updateIncome(userId: string, incomeId: string, income: Partial<CreateIncomeRequest>): Observable<Income> {
    return this.http.patch<Income>(`${this.apiUrl}/users/${userId}/income/${incomeId}`, income);
  }

  deleteIncome(userId: string, incomeId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/users/${userId}/income/${incomeId}`);
  }
} 