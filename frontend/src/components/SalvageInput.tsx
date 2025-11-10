import React, { useState } from 'react';
import { SalvageItem } from '../types';

interface SalvageInputProps {
  salvageItems: SalvageItem[];
  onAddItem: (item: SalvageItem) => void;
  onAddItems: (items: SalvageItem[]) => void;
  onRemoveItem: (index: number) => void;
  onClear: () => void;
}

const COMMON_SALVAGE = [
  'Tripped Power Circuit',
  'Charred Micro Circuit',
  'Fried Interface Circuit',
  'Contaminated Nanite Compound',
  'Armor Plates',
  'Burned Logic Circuit',
];

const SalvageInput: React.FC<SalvageInputProps> = ({
  salvageItems,
  onAddItem,
  onAddItems,
  onRemoveItem,
  onClear,
}) => {
  const [name, setName] = useState('');
  const [quantity, setQuantity] = useState('');
  const [pasteText, setPasteText] = useState('');
  const [showPaste, setShowPaste] = useState(false);

  const handleAdd = () => {
    if (name.trim() && quantity && parseInt(quantity) > 0) {
      onAddItem({
        name: name.trim(),
        quantity: parseInt(quantity),
      });
      setName('');
      setQuantity('');
    }
  };

  const handleQuickAdd = (itemName: string) => {
    setName(itemName);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleAdd();
    }
  };

  const parsePastedItems = (text: string): SalvageItem[] => {
    const items: SalvageItem[] = [];
    const lines = text.split('\n');

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed) continue;

      // Try different formats:
      // EVE multi-column format: "Item Name    3    Salvaged Materials    Material    0.03 m3    13,065.96 ISK"
      // "Item Name x123"
      // "Item Name  123"
      // "Item Name	123"

      let match = null;

      // Match EVE detailed format with multiple columns (item name, qty, category, type, volume, ISK)
      // Example: "Broken Drone Transceiver    3    Salvaged Materials    Material            0.03 m3    13,065.96 ISK"
      // This format uses tabs and/or multiple spaces between columns
      // Try to match: name, then (tab OR 2+ spaces), then number, then (tab OR space), then rest
      match = trimmed.match(/^(.+?)(?:\t+|\s{2,})(\d+)(?:\t|\s).+$/);

      if (!match) {
        // Also try just: name, whitespace, number, whitespace, anything else
        match = trimmed.match(/^(.+?)\s+(\d+)\s+\S.+$/);
      }

      if (!match) {
        // Match "x" separator (e.g., "Tripped Power Circuit x120")
        match = trimmed.match(/^(.+?)\s+x\s*(\d+)$/i);
      }

      if (!match) {
        // Match tab or multiple spaces at end (e.g., "Tripped Power Circuit	120" or "Tripped Power Circuit  120")
        match = trimmed.match(/^(.+?)[\t\s]{2,}(\d+)$/);
      }

      if (!match) {
        // Match single space with number at end (e.g., "Tripped Power Circuit 120")
        match = trimmed.match(/^(.+?)\s+(\d+)$/);
      }

      if (match) {
        const itemName = match[1].trim();
        const qty = parseInt(match[2]);

        if (itemName && qty > 0) {
          items.push({
            name: itemName,
            quantity: qty,
          });
        }
      }
    }

    return items;
  };

  const handlePasteItems = () => {
    const items = parsePastedItems(pasteText);

    if (items.length === 0) {
      alert('No valid items found. Supported formats:\n\n' +
        '• EVE multi-column: Item Name    3    Salvaged Materials...\n' +
        '• x-separator: Item Name x123\n' +
        '• Space/tab separated: Item Name  123');
      return;
    }

    onAddItems(items);
    setPasteText('');
    setShowPaste(false);
  };

  return (
    <div className="card">
      <h2 className="text-2xl font-bold mb-4 text-eve-accent">Salvage & Loot Input</h2>

      {/* Paste from Inventory Section */}
      <div className="mb-4 p-3 bg-eve-gray rounded">
        <button
          onClick={() => setShowPaste(!showPaste)}
          className="w-full text-left font-semibold text-eve-accent hover:text-eve-accent/80 transition-colors flex justify-between items-center"
        >
          <span>📋 Paste from EVE Inventory (Salvage, Modules, Ammo)</span>
          <span className="text-sm">{showPaste ? '▼' : '▶'}</span>
        </button>

        {showPaste && (
          <div className="mt-3">
            <textarea
              className="input w-full h-32 font-mono text-sm"
              value={pasteText}
              onChange={(e) => setPasteText(e.target.value)}
              placeholder="Paste salvage, loot, modules, or ammo from EVE inventory...&#10;&#10;Supported formats:&#10;Broken Drone Transceiver    3    Salvaged Materials    Material    0.03 m3    13,065.96 ISK&#10;Light Neutron Blaster I    5    Hybrid Turret    Module    5 m3    15,000 ISK&#10;Antimatter Charge M x1000"
            />
            <div className="flex gap-2 mt-2">
              <button onClick={handlePasteItems} className="btn-primary flex-1">
                Import Items
              </button>
              <button
                onClick={() => {
                  setPasteText('');
                  setShowPaste(false);
                }}
                className="btn-secondary"
              >
                Cancel
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Manual Input Section */}
      <div className="mb-4">
        <label className="label">Item Name</label>
        <input
          type="text"
          className="input w-full"
          value={name}
          onChange={(e) => setName(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Enter salvage item name"
          list="salvage-items"
        />
        <datalist id="salvage-items">
          {COMMON_SALVAGE.map((item) => (
            <option key={item} value={item} />
          ))}
        </datalist>
      </div>

      <div className="mb-4">
        <label className="label">Quantity</label>
        <input
          type="number"
          className="input w-full"
          value={quantity}
          onChange={(e) => setQuantity(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Enter quantity"
          min="1"
        />
      </div>

      <div className="flex gap-2 mb-4">
        <button onClick={handleAdd} className="btn-primary flex-1">
          Add Item
        </button>
        {salvageItems.length > 0 && (
          <button onClick={onClear} className="btn-secondary">
            Clear All
          </button>
        )}
      </div>

      <div className="mb-4">
        <label className="label">Quick Add</label>
        <div className="flex flex-wrap gap-2">
          {COMMON_SALVAGE.map((item) => (
            <button
              key={item}
              onClick={() => handleQuickAdd(item)}
              className="text-xs bg-gray-700 hover:bg-gray-600 text-gray-100 px-2 py-1 rounded transition-colors"
            >
              {item}
            </button>
          ))}
        </div>
      </div>

      {salvageItems.length > 0 && (
        <div>
          <h3 className="text-lg font-semibold mb-2">Current Items</h3>
          <div className="space-y-2">
            {salvageItems.map((item, index) => (
              <div
                key={index}
                className="flex justify-between items-center bg-eve-gray p-3 rounded"
              >
                <div>
                  <span className="font-medium">{item.name}</span>
                  <span className="text-gray-400 ml-2">×{item.quantity}</span>
                </div>
                <button
                  onClick={() => onRemoveItem(index)}
                  className="text-red-400 hover:text-red-300 transition-colors"
                >
                  Remove
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default SalvageInput;
