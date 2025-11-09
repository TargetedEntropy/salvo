import React, { useState } from 'react';
import { BuildableItem } from '../types';

interface BuildableItemsListProps {
  items: BuildableItem[];
}

const BuildableItemsList: React.FC<BuildableItemsListProps> = ({ items }) => {
  const [expandedIndex, setExpandedIndex] = useState<number | null>(null);

  if (items.length === 0) {
    return null;
  }

  const formatISK = (value: number): string => {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(value);
  };

  const toggleExpand = (index: number) => {
    setExpandedIndex(expandedIndex === index ? null : index);
  };

  return (
    <div className="card">
      <h2 className="text-2xl font-bold mb-4 text-eve-accent">Buildable Items</h2>

      <div className="space-y-3">
        {items.map((item, index) => (
          <div
            key={item.product_type_id}
            className="bg-eve-gray rounded-lg overflow-hidden border border-gray-700"
          >
            <div
              className="p-4 cursor-pointer hover:bg-gray-800 transition-colors"
              onClick={() => toggleExpand(index)}
            >
              <div className="flex justify-between items-start mb-2">
                <div className="flex-1">
                  <h3 className="text-lg font-semibold text-eve-accent mb-1">
                    {item.product_name}
                  </h3>
                  <div className="flex gap-4 text-sm text-gray-400">
                    <span>
                      Match:{' '}
                      <span className={item.match_percentage === 100 ? 'text-green-400' : 'text-yellow-400'}>
                        {item.match_percentage.toFixed(0)}%
                      </span>
                    </span>
                    <span>
                      Status:{' '}
                      <span className={item.can_build ? 'text-green-400' : 'text-red-400'}>
                        {item.can_build ? 'Can Build' : 'Missing Materials'}
                      </span>
                    </span>
                  </div>
                </div>
                <div className="text-right">
                  <div className="text-2xl font-bold text-eve-gold">
                    {formatISK(item.estimated_profit)} ISK
                  </div>
                  <div className="text-sm text-gray-400">
                    {item.profit_margin.toFixed(1)}% margin
                  </div>
                </div>
              </div>

              <div className="flex justify-between items-center text-sm">
                <div className="flex gap-4 text-gray-300">
                  <span>
                    Product: <span className="text-gray-100">{formatISK(item.product_price)} ISK</span>
                  </span>
                  <span>
                    Materials: <span className="text-gray-100">{formatISK(item.material_cost)} ISK</span>
                  </span>
                </div>
                <button className="text-eve-accent hover:text-eve-accent/80">
                  {expandedIndex === index ? '▼' : '▶'} Details
                </button>
              </div>
            </div>

            {expandedIndex === index && (
              <div className="border-t border-gray-700 p-4 bg-eve-darker">
                <h4 className="font-semibold mb-2">Build Details</h4>
                <div className="grid grid-cols-2 gap-4 mb-4 text-sm">
                  <div>
                    <span className="text-gray-400">Product Price:</span>
                    <span className="ml-2 text-gray-100">{formatISK(item.product_price)} ISK</span>
                  </div>
                  <div>
                    <span className="text-gray-400">Material Cost:</span>
                    <span className="ml-2 text-gray-100">{formatISK(item.material_cost)} ISK</span>
                  </div>
                  <div>
                    <span className="text-gray-400">Estimated Profit:</span>
                    <span className="ml-2 text-eve-gold">{formatISK(item.estimated_profit)} ISK</span>
                  </div>
                  <div>
                    <span className="text-gray-400">Profit Margin:</span>
                    <span className="ml-2 text-green-400">{item.profit_margin.toFixed(2)}%</span>
                  </div>
                </div>

                {item.missing_materials.length > 0 && (
                  <div>
                    <h4 className="font-semibold mb-2 text-red-400">Missing Materials</h4>
                    <div className="space-y-2">
                      {item.missing_materials.map((material) => (
                        <div
                          key={material.type_id}
                          className="bg-eve-gray p-2 rounded flex justify-between text-sm"
                        >
                          <span>{material.name}</span>
                          <span className="text-red-400">
                            Need {material.needed.toLocaleString()}, Missing{' '}
                            {material.missing.toLocaleString()}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default BuildableItemsList;
