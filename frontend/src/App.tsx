import React, { useState } from 'react';
import SalvageInput from './components/SalvageInput';
import MaterialsList from './components/MaterialsList';
import BuildableItemsList from './components/BuildableItemsList';
import { analyzeSalvage } from './api';
import { SalvageItem, AnalysisResponse } from './types';

function App() {
  const [salvageItems, setSalvageItems] = useState<SalvageItem[]>([]);
  const [reprocessingEfficiency, setReprocessingEfficiency] = useState<number>(0.5);
  const [analysisResult, setAnalysisResult] = useState<AnalysisResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleAddItem = (item: SalvageItem) => {
    setSalvageItems([...salvageItems, item]);
  };

  const handleAddItems = (items: SalvageItem[]) => {
    setSalvageItems([...salvageItems, ...items]);
  };

  const handleRemoveItem = (index: number) => {
    setSalvageItems(salvageItems.filter((_, i) => i !== index));
  };

  const handleClear = () => {
    setSalvageItems([]);
    setAnalysisResult(null);
    setError(null);
  };

  const handleAnalyze = async () => {
    if (salvageItems.length === 0) {
      setError('Please add at least one salvage item');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const result = await analyzeSalvage({
        salvage_items: salvageItems,
        reprocessing_efficiency: reprocessingEfficiency,
      });
      setAnalysisResult(result);
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || 'Failed to analyze salvage');
      setAnalysisResult(null);
    } finally {
      setLoading(false);
    }
  };

  const getEfficiencyLabel = (efficiency: number): string => {
    if (efficiency >= 0.69) return 'Perfect Skills (69.575%)';
    if (efficiency >= 0.6) return 'Advanced Skills';
    if (efficiency >= 0.55) return 'Intermediate Skills';
    return 'Basic Skills (50%)';
  };

  return (
    <div className="min-h-screen">
      {/* Header */}
      <header className="bg-eve-dark border-b border-gray-800 shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <h1 className="text-4xl font-bold text-eve-accent mb-2">
            Salvo
          </h1>
          <p className="text-gray-400">
            Serpentis Salvage Industrial Planner - Optimize your salvage profits
          </p>
        </div>
      </header>

      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Left Column - Input */}
          <div className="lg:col-span-1 space-y-6">
            <SalvageInput
              salvageItems={salvageItems}
              onAddItem={handleAddItem}
              onAddItems={handleAddItems}
              onRemoveItem={handleRemoveItem}
              onClear={handleClear}
            />

            <div className="card">
              <h2 className="text-xl font-bold mb-4 text-eve-accent">
                Reprocessing Efficiency
              </h2>
              <div className="mb-2">
                <label className="label">
                  Efficiency: {(reprocessingEfficiency * 100).toFixed(2)}%
                </label>
                <input
                  type="range"
                  min="0.5"
                  max="0.69575"
                  step="0.001"
                  value={reprocessingEfficiency}
                  onChange={(e) => setReprocessingEfficiency(parseFloat(e.target.value))}
                  className="w-full"
                />
                <div className="text-sm text-gray-400 mt-1">
                  {getEfficiencyLabel(reprocessingEfficiency)}
                </div>
              </div>

              <button
                onClick={handleAnalyze}
                disabled={loading || salvageItems.length === 0}
                className="btn-primary w-full mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? 'Analyzing...' : 'Analyze Salvage'}
              </button>
            </div>

            {error && (
              <div className="bg-red-900/30 border border-red-500 text-red-200 p-4 rounded">
                <p className="font-semibold">Error</p>
                <p className="text-sm">{error}</p>
              </div>
            )}
          </div>

          {/* Right Column - Results */}
          <div className="lg:col-span-2 space-y-6">
            {analysisResult ? (
              <>
                <MaterialsList
                  materials={analysisResult.materials}
                  totalValue={analysisResult.total_material_value}
                />
                <BuildableItemsList items={analysisResult.buildable_items} />
                {analysisResult.unknown_items && analysisResult.unknown_items.length > 0 && (
                  <div className="card border-2 border-yellow-600/50 bg-yellow-900/20">
                    <h3 className="text-xl font-bold mb-3 text-yellow-400 flex items-center">
                      <svg className="w-6 h-6 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                      </svg>
                      Unknown Items (Cannot Reprocess)
                    </h3>
                    <p className="text-sm text-gray-400 mb-3">
                      The following items are not in our database or cannot be reprocessed:
                    </p>
                    <div className="space-y-2">
                      {analysisResult.unknown_items.map((item, index) => (
                        <div
                          key={index}
                          className="bg-eve-gray/50 p-3 rounded border border-yellow-600/30"
                        >
                          <div className="flex justify-between items-center">
                            <span className="font-medium text-yellow-200">{item.name}</span>
                            <span className="text-gray-400">×{item.quantity}</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </>
            ) : (
              <div className="card text-center py-12">
                <svg
                  className="mx-auto h-24 w-24 text-gray-600 mb-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                  />
                </svg>
                <h3 className="text-xl font-semibold text-gray-400 mb-2">
                  No Analysis Yet
                </h3>
                <p className="text-gray-500">
                  Add salvage items and click "Analyze Salvage" to see results
                </p>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Footer */}
      <footer className="bg-eve-dark border-t border-gray-800 mt-12">
        <div className="container mx-auto px-4 py-6 text-center text-gray-500 text-sm">
          <p>
            EVE Online and all related assets are property of CCP Games.
          </p>
          <p className="mt-1">
            This tool is not affiliated with or endorsed by CCP Games.
          </p>
        </div>
      </footer>
    </div>
  );
}

export default App;
