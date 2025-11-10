export interface SalvageItem {
  name: string;
  quantity: number;
}

export interface Material {
  type_id: number;
  name: string;
  quantity: number;
  unit_price: number;
  total_value: number;
}

export interface MissingMaterial {
  type_id: number;
  name: string;
  needed: number;
  available: number;
  missing: number;
  cost_to_buy: number;
}

export interface BuildableItem {
  product_type_id: number;
  product_name: string;
  match_percentage: number;
  can_build: boolean;
  estimated_profit: number;
  profit_margin: number;
  product_price: number;
  material_cost: number;
  missing_materials: MissingMaterial[];
}

export interface UnknownItem {
  name: string;
  quantity: number;
}

export interface AnalysisResponse {
  materials: Material[];
  total_material_value: number;
  buildable_items: BuildableItem[];
  reprocessing_efficiency_used: number;
  unknown_items: UnknownItem[];
}

export interface AnalysisRequest {
  salvage_items: SalvageItem[];
  reprocessing_efficiency: number;
}
