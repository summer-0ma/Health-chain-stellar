import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  Index,
} from 'typeorm';

@Entity('blood_units')
@Index(['unitNumber'], { unique: true })
@Index(['bloodType', 'bankId'])
export class BloodUnitEntity {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ type: 'varchar', length: 80, unique: true })
  unitNumber: string;

  @Column({ type: 'bigint', nullable: true })
  blockchainUnitId?: number;

  @Column({ type: 'varchar', length: 255 })
  blockchainTransactionHash: string;

  @Column({ type: 'varchar', length: 5 })
  bloodType: string;

  @Column({ type: 'int' })
  quantityMl: number;

  @Column({ type: 'varchar', length: 100, nullable: true })
  donorId?: string;

  @Column({ type: 'varchar', length: 70 })
  bankId: string;

  @Column({ type: 'timestamp' })
  expirationDate: Date;

  @Column({ type: 'varchar', length: 80, nullable: true })
  registeredBy?: string;

  @Column({ type: 'text' })
  barcodeData: string;

  @Column({ type: 'jsonb', nullable: true })
  metadata?: Record<string, unknown>;

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
