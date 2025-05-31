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
  description?: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewIncome {
  user_id: string;
  source: string;
  amount: string;
  date: string;
  description?: string | null;
}

export interface UpdateIncome {
  source?: string | null;
  amount?: string | null;
  date?: string | null;
  description?: string | null;
  updated_at?: string | null;
}

@Injectable({
  providedIn: 'root'
})
export class IncomeService {
  private apiUrl = environment.apiUrl;

  constructor(private http: HttpClient) {}

  // GET /api/incomes (all incomes)
  getAllIncomes(): Observable<Income[]> {
    return this.http.get<Income[]>(`${this.apiUrl}/api/incomes`);
  }

  // GET /api/incomes/user/{user_id}
  getIncomesByUserId(userId: string): Observable<Income[]> {
    return this.http.get<Income[]>(`${this.apiUrl}/api/incomes/user/${userId}`);
  }

  // POST /api/incomes
  createIncome(income: NewIncome): Observable<Income> {
    return this.http.post<Income>(`${this.apiUrl}/api/incomes`, income);
  }

  // PUT /api/incomes/{income_id}
  updateIncome(incomeId: string, update: UpdateIncome): Observable<Income> {
    return this.http.put<Income>(`${this.apiUrl}/api/incomes/${incomeId}`, update);
  }

  // DELETE /api/incomes/{income_id}
  deleteIncome(incomeId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/api/incomes/${incomeId}`);
  }
} 