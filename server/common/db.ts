import { Sequelize, DataTypes } from "sequelize";
import { Eras, ValidatorEraPayout, QueryStatus } from "./models";

export function initResultDb(db: Sequelize): void {
  Eras.init(
    {
      index: {
        type: DataTypes.INTEGER,
        allowNull: false,
        primaryKey: true,
      },
      startBlock: {
        type: DataTypes.INTEGER,
        allowNull: true,
      },
      startBlockTimestamp: {
        type: DataTypes.BIGINT,
        allowNull: true,
      },
      endBlock: {
        type: DataTypes.INTEGER,
        allowNull: true,
      },
      endBlockTimestamp: {
        type: DataTypes.BIGINT,
        allowNull: true,
      },
      totalRewards: {
        type: DataTypes.STRING,
        allowNull: true,
      },
      totalSlashes: {
        type: DataTypes.STRING,
        allowNull: true,
      },
    },
    {
      sequelize: db,
      indexes: [
        {
          fields: ["index"],
        },
      ],
    }
  );
}
